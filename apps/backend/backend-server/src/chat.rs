use actix_web::Result;
use anyhow::Error;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::pin::Pin;

#[derive(Debug, Deserialize)]
struct ChatChunkDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatChunkChoice {
    delta: ChatChunkDelta,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChunk {
    choices: Vec<ChatChunkChoice>,
}

pub async fn send_request(
    question: String,
    context: String,
) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>>, Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let api_key = env::var("OPENAI_API_KEY")?;

    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "assistant",
                "content": "You are a CV chat robot for Dennis Jensen. Everyone that is going to ask you a question will want to know something about the capabilities of Dennis Jensen aka the Digital Craftsman. Please answer the questions asked and answer how the digital craftsman - Dennis Jensen fits into the role, task or job the question is asking about. You are provided with context of what projects and positions dennis have been a part of refer to that context when you can. Do not say that Dennis Jensen can do anything unrelated to the context you are provided. Especially which programming languages he can use."
            },
            {
                "role": "system",
                "content": context
            },
            {
                "role": "user",
                "content": question
            }
        ],
        "stream": true
    });

    let client = Client::new();
    let res = client
        .post(url)
        .body(body.to_string())
        .header("Content-Type", "application/json")
        .bearer_auth(api_key)
        .send()
        .await?;

    let stream = res.bytes_stream().map(|item| match item {
        Ok(data) => {
            let s = String::from_utf8_lossy(&data);
            let mut output = Bytes::new();
            for p in s.split("\n\n") {
                if let Some(p) = p.strip_prefix("data: ") {
                    if p == "[DONE]" {
                        return Ok(Bytes::from_static(b""));
                    }

                    let d = serde_json::from_str::<ChatCompletionChunk>(p).unwrap_or_else(|_| {
                        println!("Couldn't parse: {}", p);
                        ChatCompletionChunk { choices: vec![] }
                    });

                    let default_choice = &ChatChunkChoice {
                        delta: ChatChunkDelta { content: None },
                    };

                    let c = d.choices.get(0).unwrap_or(default_choice);

                    if let Some(content) = &c.delta.content {
                        output = Bytes::from(content.to_string());
                    }
                }
            }
            Ok(output)
        }
        Err(_) => Ok(Bytes::from_static(b"Error")),
    });

    Ok(Box::pin(stream))
}
