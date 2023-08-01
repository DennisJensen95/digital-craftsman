use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Starting the server...");

    HttpServer::new(|| {
        App::new()
            .service(
                fs::Files::new("/", "./digital-craftsman")
                    .index_file("index.html")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .route("/health", web::get().to(health))
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
