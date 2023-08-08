pub mod qdrant {
    use anyhow::{bail, Result};
    use log;
    use qdrant_client::prelude::*;
    use qdrant_client::qdrant::vectors_config::Config;
    use qdrant_client::qdrant::{
        CollectionOperationResponse, CreateCollection, SearchPoints, VectorParams, VectorsConfig,
    };
    use serde_json::json;

    pub struct DocumentData {
        // List of strings (e.g., extracted content from a document).
        pub sections: Vec<String>,

        // Corresponding vector embeddings for each string.
        pub embeddings: Vec<Vec<f32>>,

        // Name of the source file.
        pub source_files: Vec<String>,
    }

    impl DocumentData {
        /// Creates a new DocumentData instance.
        ///
        /// # Arguments
        ///
        /// * `contents` - A vector of strings from a document.
        /// * `embeddings` - A vector of embeddings corresponding to each string.
        /// * `source_files` - The name of the source files.
        pub fn new(
            sections: Vec<String>,
            embeddings: Vec<Vec<f32>>,
            source_files: Vec<String>,
        ) -> Result<Self> {
            if sections.len() == embeddings.len() && sections.len() == source_files.len() {
                Ok(Self {
                    sections,
                    embeddings,
                    source_files,
                })
            } else {
                bail!("Contents, embeddings, and source files must have the same length.");
            }
        }

        pub async fn upload_to_qdrant(&self) -> Result<()> {
            log::info!("Uploading to Qdrant");

            let client = match self.make_client() {
                Ok(client) => client,
                Err(_) => bail!("Error creating client"),
            };

            log::info!("Client created");

            let collection_name = "digital-craftsman";
            let collection = match self.make_collection(collection_name, &client).await {
                Ok(collection) => collection,
                Err(err) => bail!("Error creating collection: {:?}", err),
            };

            log::info!("Creating collection made changes: {:?}", collection.result);

            let points = self.make_point_structs();
            client
                .upsert_points_blocking(collection_name, points, None)
                .await?;

            let search_result = client
                .search_points(&SearchPoints {
                    collection_name: collection_name.into(),
                    vector: self.embeddings[0].clone(),
                    limit: 10,
                    with_payload: Some(true.into()),
                    ..Default::default()
                })
                .await?;
            log::info!("Search result: {:?}", search_result);

            Ok(())
        }

        pub fn make_point_structs(&self) -> Vec<PointStruct> {
            let mut point_structs = Vec::new();

            for (index, embedding) in self.embeddings.iter().enumerate() {
                let payload: Payload = json!(
                    {
                        "section": self.sections[index],
                        "source_file": self.source_files[index]
                    }
                )
                .try_into()
                .unwrap();

                point_structs.push(PointStruct::new(index as u64, embedding.clone(), payload));
            }

            point_structs
        }

        async fn make_collection(
            &self,
            collection_name: &str,
            client: &QdrantClient,
        ) -> Result<CollectionOperationResponse> {
            client.delete_collection(collection_name).await?;

            let collection = client
                .create_collection(&CreateCollection {
                    collection_name: collection_name.into(),
                    vectors_config: Some(VectorsConfig {
                        config: Some(Config::Params(VectorParams {
                            size: self.embeddings[0].len() as u64,
                            distance: Distance::Cosine.into(),
                            ..Default::default()
                        })),
                    }),
                    ..Default::default()
                })
                .await;

            collection
        }

        fn make_client(&self) -> Result<QdrantClient> {
            let client =
                QdrantClient::from_url("https://3410221d-e3ce-4368-92d9-41fa898a5f19.eu-central-1-0.aws.cloud.qdrant.io:6334")
                    // using an env variable for the API KEY for example
                    .with_api_key(std::env::var("QDRANT_API_KEY"))
                    .build()?;
            Ok(client)
        }
    }

    // Add any associated functions, tests, or other structs as needed.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_data_creation_valid() {
        let contents = vec!["test_content".to_string()];
        let embeddings = vec![vec![0.5]];
        let source_files = vec!["source.txt".to_string()];

        let data = qdrant::DocumentData::new(contents, embeddings, source_files);
        assert!(data.is_ok());
    }

    #[test]
    fn test_document_data_creation_invalid() {
        let contents = vec!["test_content".to_string(), "extra_content".to_string()];
        let embeddings = vec![vec![0.5]];
        let source_files = vec!["source.txt".to_string()];

        let data = qdrant::DocumentData::new(contents, embeddings, source_files);
        assert!(data.is_err());
    }

    #[test]
    fn test_make_point_structs() {
        let sections = vec!["test_content".to_string()];
        let embeddings = vec![vec![0.5]];
        let source_files = vec!["source.txt".to_string()];

        let data = qdrant::DocumentData::new(sections, embeddings, source_files).unwrap();
        let point_structs = data.make_point_structs();

        assert_eq!(point_structs.len(), 1);
        assert_eq!(point_structs[0].id.is_some(), true);
    }
}
