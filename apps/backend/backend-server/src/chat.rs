use actix_web::Error;
use async_openai::{
    types::ChatCompletionRequestMessage, types::CreateChatCompletionRequestArgs, types::Role,
    Client,
};

fn read_files_in_directory(path: &str) -> String {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path = path.to_str().unwrap();
        files.push(path.to_string());
    }

    // Join all files into a single string
    files.join("\n")
}

pub async fn send_request(question: String) -> Result<String, Error> {
    // Create a OpenAI client with api key from env var OPENAI_API_KEY and default base url.
    let client = Client::new();

    let tech_skills = read_files_in_directory("markdown-files/tech-skills");
    let work_experience = read_files_in_directory("markdown-files/resume");

    let messages = vec![
        ChatCompletionRequestMessage {
            role: Role::Assistant,
            content: Some("You are a CV chat robot for Dennis Jensen. Everyone that is going to ask you a question will want to know something about the capabilities of Dennis Jensen aka the Digital Craftsman. Please answer the questions asked and answer how the digital craftsman - Dennis Jensen fits into the role, task or job the question is asking about. You are provided with context of what projects and positions dennis have been a part of refer to that context when you can. Do not say that Dennis Jensen can do anything unrelated to the context you are provided. Especially which programming languages he can use.".to_string()),
            name: None,
            function_call: None,
        },
        ChatCompletionRequestMessage {
            role: Role::System,
            content: Some(tech_skills),
            name: None,
            function_call: None,
        },
        ChatCompletionRequestMessage {
            role: Role::System,
            content: Some(work_experience),
            name: None,
            function_call: None,
        },
        ChatCompletionRequestMessage {
            role: Role::User,
            content: Some(question),
            name: None,
            function_call: None,
        },
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(messages)
        .build()
        .unwrap();

    let response = client
        .chat() // Get the API "group" (completions, images, etc.) from the client
        .create(request) // Make the API call in that "group"
        .await
        .unwrap();

    let response_string = response
        .choices
        .first()
        .unwrap()
        .message
        .clone()
        .content
        .unwrap();

    Ok(response_string)
}
