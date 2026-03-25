use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    time::Instant,
};

use log::{debug, warn};
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{PointStruct, QueryPointsBuilder, UpsertPointsBuilder},
};
use rig::{embeddings::EmbeddingModel, loaders::FileLoader};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use text_splitter::ChunkConfig;

use sha2::{Sha256, digest::Digest};

use crate::INFO_COLLECTION;

const EMBED_CHUNK_SIZE: usize = 5;

pub async fn incremental_index<T: AsRef<Path>>(
    client: &Qdrant,
    path: T,
    embedding_model: &impl EmbeddingModel,
) -> anyhow::Result<()> {
    debug!("Loading embeddings");
    let embeddings = load_embeddings(path)?;
    debug!("Loaded embeddings");
    let t = Instant::now();
    // grab ALL the points.
    let query_result = client
        .query(QueryPointsBuilder::new(INFO_COLLECTION))
        .await?;
    debug!("Loaded points in {} seconds", t.elapsed().as_secs_f32());
    let mut map = HashMap::new();
    let mut points_to_embed = vec![];
    let t = Instant::now();

    for i in query_result.result.iter() {
        let content = i.payload["content_hash"].as_str();
        let file = i.payload["path_hash"].as_str();

        if let Some(content) = content
            && let Some(file) = file
        {
            map.insert(file, content);
        } else {
            warn!("Missing fields for vector entry!");
        }
    }

    debug!("Processed points in {} seconds", t.elapsed().as_secs_f32());
    debug!("Found {} points", map.len());

    for i in embeddings.into_iter() {
        if let Some(e) = map.get(&i.path_hash)
            && *e == &i.content_hash
        {
            // all good
        } else {
            points_to_embed.push(i);
        }
    }

    debug!("Found {} chunks to embed", points_to_embed.len());

    if points_to_embed.is_empty() {
        warn!("No points found to embed");
        return Ok(());
    }

    upsert_points(client, &points_to_embed, embedding_model).await?;

    Ok(())
}

async fn upsert_points(
    client: &Qdrant,
    embeddings: &[Embedding],
    embedding_model: &impl EmbeddingModel,
) -> anyhow::Result<()> {
    let mut vectors = vec![];

    for i in embeddings.rchunks(EMBED_CHUNK_SIZE) {
        let mut inner_vectors = embedding_model
            .embed_texts(i.into_iter().map(|f| f.content.clone()))
            .await?;
        vectors.append(&mut inner_vectors);
    }

    assert_eq!(vectors.len(), embeddings.len());

    let mut points = vec![];
    for (vector, embedding) in vectors.iter().zip(embeddings) {
        assert_ne!(embedding.path_hash.len(), 0);
        let i = PointStruct::new(
            embedding.path_hash.clone(),
            vector.vec.iter().map(|f| *f as f32).collect::<Vec<_>>(),
            Payload::try_from(serde_json::to_value(embedding)?)?,
        );

        points.push(i);
    }

    client
        .upsert_points_chunked(UpsertPointsBuilder::new(INFO_COLLECTION, points), 20)
        .await?;

    Ok(())
}

/// Load documents, split them up, and return ready embeddings.
fn load_embeddings<T: AsRef<Path>>(path: T) -> anyhow::Result<Vec<Embedding>> {
    let documents = get_documents(path)?;

    let mut out = vec![];

    for (path, content) in documents {
        let path_hash = Sha256::digest(path.to_string_lossy().as_bytes());
        let path_hash = format!("{:x}", path_hash);

        let chunks = split_text(&content);
        let chunks = merge_chunks(chunks);

        for chunk in chunks {
            let content_hash = Sha256::digest(&chunk);
            let content_hash = format!("{:x}", content_hash);

            let i = Embedding {
                path_hash: path_hash.clone(),
                content: chunk,
                content_hash,
                headers: "".to_string(),
            };

            out.push(i)
        }
    }

    Ok(out)
}

fn get_documents<T: AsRef<Path>>(path: T) -> anyhow::Result<Vec<(PathBuf, String)>> {
    debug!("Loading documents");
    let p = path.as_ref().join("**/*.md");
    let p = p
        .to_str()
        .expect("Should work.. what idiot made this library");
    let glob_loader = FileLoader::with_glob(p)?;

    let i = glob_loader
        .read_with_path()
        .ignore_errors()
        .into_iter()
        .collect::<Vec<_>>();

    Ok(i)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Embedding {
    /// Hash of path.
    path_hash: String,

    content: String,
    content_hash: String,
    headers: String,
}

/// Split the text into chunks
fn split_text(s: &str) -> Vec<String> {
    let tokenizer = tiktoken_rs::cl100k_base_singleton();
    let config = ChunkConfig::new(200..512)
        .with_sizer(tokenizer)
        .with_overlap(100)
        .expect("Should work");
    let chunks = text_splitter::MarkdownSplitter::new(config)
        .chunks(s)
        .collect::<Vec<&str>>();

    let i = merge_chunks(
        chunks
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>(),
    );

    i
}
/// Merge small chunks.
fn merge_chunks(chunks: Vec<String>) -> Vec<String> {
    let tokenizer = tiktoken_rs::cl100k_base_singleton();
    let min = 200;

    let mut merged = Vec::new();
    let mut current = String::new();

    for chunk in chunks {
        let c = chunk.trim();
        let tokens = tokenizer.encode_ordinary(c).len();

        if tokens < min && !merged.is_empty() {
            current.push_str("\n\n");
            current.push_str(c);
        } else {
            if !current.is_empty() {
                merged.push(current.trim().to_string());
            }
            current = chunk;
        }
    }

    if !current.is_empty() {
        merged.push(current.trim().to_string());
    }

    merged
}
