// Standard library
use std::io;

// Third party imports
use anyhow::{bail, Result};
use clap::Parser;
use rust_bert::{
    pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType},
    RustBertError,
};

// Application imports
mod qdrant_connector;
use qdrant_connector::qdrant::DocumentData;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to read files from
    #[arg(short, long)]
    name: String,
}

fn section_string(input: &str, max_chars: usize) -> Vec<String> {
    let mut sections = Vec::new();
    let mut start = 0;

    while start < input.len() {
        // Find the maximum allowable end position
        let mut end = start + max_chars;
        if end > input.len() {
            end = input.len();
        }

        // Adjust the end position if we are close to the max_chars boundary
        if end - start == max_chars {
            while end > start
                && !['.', '!', '?', ' '].contains(&input.chars().nth(end - 1).unwrap_or_default())
            {
                end -= 1;
            }
        }

        // If we're in the middle of a sentence (but on a space), backtrack to the previous sentence end
        if end > start && [' '].contains(&input.chars().nth(end - 1).unwrap_or_default()) {
            let mut temp_end = end;
            while temp_end > start
                && !['.', '!', '?'].contains(&input.chars().nth(temp_end - 1).unwrap_or_default())
            {
                temp_end -= 1;
            }
            if temp_end != start {
                end = temp_end;
            }
        }

        // If no valid ending was found, we're probably dealing with very long words or sentences,
        // so force an end position
        if end == start {
            end = start + max_chars;
            if end > input.len() {
                end = input.len();
            }
        }

        // Push the section to the result vector
        sections.push(input[start..end].trim().to_string());

        // Update the start for the next iteration
        start = end;
    }

    sections
}

fn read_files_in_directory(path: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path = path
            .to_str()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Non-unicode path"))?;

        files.push(path.to_string());
    }
    Ok(files)
}

fn parse_files(files: Vec<String>) -> Result<DocumentData> {
    let mut sections = Vec::new();
    let mut embeddings = Vec::new();
    let mut file_names = Vec::new();

    for file in files {
        println!("Parsing file: {}", file);
        let file_characters = match std::fs::read_to_string(&file) {
            Ok(contents) => contents,
            Err(_) => continue,
        };
        let new_sections = section_string(&file_characters, 1500);
        let number_of_sections = new_sections.len();

        for _ in 0..number_of_sections {
            file_names.push(file.to_string());
        }
        match compute_embeddings(&new_sections) {
            Ok(embeds) => embeddings.extend(embeds),
            Err(_) => bail!("Error computing embeddings"),
        };

        sections.extend(new_sections);
    }

    DocumentData::new(sections, embeddings, file_names)
}

fn compute_embeddings(sections: &Vec<String>) -> Result<Vec<Vec<f32>>, RustBertError> {
    let model = SentenceEmbeddingsBuilder::remote(
        SentenceEmbeddingsModelType::DistiluseBaseMultilingualCased,
    )
    .create_model()?;

    // Generate Embeddings
    model.encode(&sections)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    log::info!("Starting program");

    let mut files = Vec::new();
    files.extend(read_files_in_directory("../../apps/frontend/src/markdown-files/resume").unwrap());
    files.extend(
        read_files_in_directory("../../apps/frontend/src/markdown-files/blog-posts").unwrap(),
    );
    files.extend(
        read_files_in_directory("../../apps/frontend/src/markdown-files/tech-skills").unwrap(),
    );

    // Set-up sentence embeddings model
    let data = match tokio::task::spawn_blocking(|| parse_files(files))
        .await
        .unwrap()
    {
        Ok(data) => data,
        Err(err) => {
            bail!("Error parsing files: {:?}", err);
        }
    };

    match data.upload_to_qdrant().await {
        Ok(_) => log::info!("Successfully uploaded to Qdrant"),
        Err(err) => bail!(err),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Write};
    use tempfile::tempdir;

    #[test]
    fn test_read_files_in_directory() {
        // Create a temporary directory.
        let dir = tempdir().unwrap();

        // Create some files in the temporary directory.
        File::create(dir.path().join("file1.txt")).unwrap();
        File::create(dir.path().join("file2.txt")).unwrap();
        File::create(dir.path().join("file3.txt")).unwrap();

        // Use the function to read files.
        let files = read_files_in_directory(dir.path().to_str().unwrap()).unwrap();

        // Check the results.
        assert_eq!(files.len(), 3);
        assert!(files.contains(&format!("{}/file1.txt", dir.path().display())));
        assert!(files.contains(&format!("{}/file2.txt", dir.path().display())));
        assert!(files.contains(&format!("{}/file3.txt", dir.path().display())));
    }

    #[test]
    fn test_parse_files() {
        // Create a temporary directory and files.
        let dir = tempdir().unwrap();
        let file_path1 = dir.path().join("file1.txt");
        let file_path2 = dir.path().join("file2.txt");

        let mut file1 = File::create(&file_path1).unwrap();
        file1.write_all(b"Hello from file1").unwrap();

        let mut file2 = File::create(&file_path2).unwrap();
        file2.write_all(b"Hello from file2").unwrap();

        let files = vec![
            file_path1.display().to_string(),
            file_path2.display().to_string(),
        ];
        let result = parse_files(files).unwrap();

        assert_eq!(
            result.sections,
            vec![
                "Hello from file1".to_string(),
                "Hello from file2".to_string()
            ]
        );
        assert_eq!(result.source_files.len(), 2);
        assert!(result
            .source_files
            .contains(&file_path1.display().to_string()));
        assert!(result
            .source_files
            .contains(&file_path2.display().to_string()));
    }

    #[test]
    fn test_section_string() {
        let text = "Hello world. This is a sample string for testing! Make sure it works?";
        let sections = section_string(&text, 20);

        assert_eq!(sections.len(), 4);
        assert_eq!(sections[0], "Hello world.");
        assert_eq!(sections[1], "This is a sample");
        assert_eq!(sections[2], "string for testing!");
        assert_eq!(sections[3], "Make sure it works?");

        // Case with a long word
        let text_with_long_word = "A veryveryverylongword indeed.";
        let sections_long = section_string(&text_with_long_word, 25);

        println!("{:?}", sections_long);
        assert_eq!(sections_long.len(), 2);
        assert_eq!(sections_long[0], "A veryveryverylongword");
        assert_eq!(sections_long[1], "indeed.");

        // Case with short input
        let short_text = "Short text.";
        let short_sections = section_string(&short_text, 50);

        assert_eq!(short_sections.len(), 1);
        assert_eq!(short_sections[0], "Short text.");

        // Hello from file
        let short_text = "Hello from file1";
        let short_sections = section_string(&short_text, 500);

        assert_eq!(short_sections.len(), 1);
        assert_eq!(short_sections[0], "Hello from file1");
    }
}
