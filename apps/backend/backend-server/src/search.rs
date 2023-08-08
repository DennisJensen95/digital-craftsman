use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{SearchPoints, SearchResponse};
use rust_bert::{
    pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType},
    RustBertError,
};

fn compute_embedding(sections: &[String]) -> Result<Vec<f32>, RustBertError> {
    let model = SentenceEmbeddingsBuilder::remote(
        SentenceEmbeddingsModelType::DistiluseBaseMultilingualCased,
    )
    .create_model()?;

    // Generate Embeddings
    let embeddings = match model.encode(sections) {
        Ok(embeddings) => embeddings,
        Err(e) => return Err(e),
    };

    Ok(embeddings[0].to_vec())
}

fn make_client() -> Result<QdrantClient> {
    let client = QdrantClient::from_url(
        "https://3410221d-e3ce-4368-92d9-41fa898a5f19.eu-central-1-0.aws.cloud.qdrant.io:6334",
    )
    .with_api_key(std::env::var("QDRANT_API_KEY"))
    .build()?;
    Ok(client)
}

fn formulate_context(search_result: SearchResponse) -> String {
    let mut context = String::from("### Context\n");
    for result in search_result.result.into_iter() {
        let payload = result.payload;
        context.push_str(&format!("{} ", payload["section"]));
    }
    context
}

pub async fn search(query: &str, collection_name: &str) -> Result<String> {
    let client = make_client()?;
    let embedding = compute_embedding(&[query.to_string()])?;

    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector: embedding,
            limit: 5,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;
    Ok(formulate_context(search_result))
}
