use actix_files as fs;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use std::time::Duration;
// Application imports
mod chat;
mod search;
use chat::send_request;
use search::search;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[derive(Serialize, Deserialize)]
struct Chat {
    question: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Starting the server...");

    let governor_conf = GovernorConfigBuilder::default()
        .period(Duration::from_secs(3600))
        .burst_size(20)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .route(
                "/chat",
                web::post().to(chat).wrap(Governor::new(&governor_conf)),
            )
            .route("/health", web::get().to(health))
            .service(
                fs::Files::new("/", "./digital-craftsman")
                    .index_file("index.html")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .default_service(web::get().to(p404))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn p404(_req: HttpRequest) -> Result<fs::NamedFile> {
    let path: std::path::PathBuf = "./digital-craftsman/index.html".into();
    Ok(fs::NamedFile::open(path)?)
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn chat(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Chat>(&body)?;

    let searches = search(&obj.question, "digital-craftsman").await.unwrap();

    let stream = send_request(obj.question, searches).await;

    match stream {
        Ok(stream) => Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "text/event-stream"))
            .streaming(stream)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, web, App};

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/", web::get().to(health))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
