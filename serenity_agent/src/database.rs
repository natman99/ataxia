use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use log::{debug, info, warn};
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{DeletePointsBuilder, PointId, PointStruct, ScrollPointsBuilder, UpsertPointsBuilder},
};
use rig::embeddings::EmbeddingModel;
use serde::{Deserialize, Serialize};
use text_splitter::ChunkConfig;

use sha2::{Sha256, digest::Digest};
use uuid::Uuid;

use crate::INFO_COLLECTION;

const EMBED_CHUNK_SIZE: usize = 5;

pub async fn incremental_index<T, E>(
    client: Arc<Qdrant>,
    path: T,
    embedding_model: &Arc<E>,
    blacklist: &[&str],
) -> anyhow::Result<()>
where
    T: AsRef<Path> + Send,
    E: EmbeddingModel + Send + Sync,
{
    let total = Instant::now();
    let t = Instant::now();
    // grab ALL the points.
    let query_result = client
        .scroll(
            ScrollPointsBuilder::new(INFO_COLLECTION)
                .with_payload(true)
                .limit(5_000),
        )
        .await?;
    debug!(
        "Loaded {} points in {} seconds",
        query_result.result.len(),
        t.elapsed().as_secs_f32()
    );
    let mut map: HashMap<&String, (PointId, &String)> = HashMap::new();
    let mut points_to_embed = vec![];
    let t = Instant::now();

    for i in query_result.result.iter() {
        if let Some(content) = i.payload.get("content_hash")
            && let Some(file) = i.payload.get("path_hash")
            && let Some(content) = content.as_str()
            && let Some(file) = file.as_str()
        {
            let Some(ref uuid) = i.id else {
                warn!("No id");
                continue;
            };

            map.insert(file, (uuid.clone(), content));
        } else {
            warn!("Missing fields for vector entry {:?}!", i);
        }
    }

    debug!("Processed points in {} seconds", t.elapsed().as_secs_f32());
    debug!("Found {} points", map.len());

    let embeddings = load_embeddings(path, blacklist).await?;

    let mut new_file_count = 0;

    for (uuid, i) in embeddings.into_iter() {
        if let Some((_uuid, e)) = map.get(&i.path_hash) {
            if *e == &i.content_hash {
                // no change required
            } else {
                // need to update

                points_to_embed.push((uuid, i.clone()));
            }
            // remove the value from the map. If there are remaining values it means we have extra points.
            map.remove(&i.path_hash);
        } else {
            // new file
            points_to_embed.push((uuid, i.clone()));
            new_file_count += 1;
        }
    }

    debug!("Found {} new files", new_file_count);

    debug!(
        "Found {} files to update",
        points_to_embed.len() - new_file_count as usize
    );

    if !map.is_empty() {
        debug!("{} deleted points. Removing points.", map.len());
        let delete = map.into_iter().map(|f| f.1.0).collect::<Vec<_>>();
        client
            .delete_points(
                DeletePointsBuilder::new(INFO_COLLECTION)
                    .points(delete)
                    .build(),
            )
            .await?;
    }

    if points_to_embed.is_empty() {
        debug!("No points found to embed");
        return Ok(());
    } else {
        debug!("Found {} chunks to embed", points_to_embed.len());
    }

    upsert_points(&client, &points_to_embed, embedding_model).await?;

    info!(
        "Completed indexing in {} seconds",
        total.elapsed().as_secs_f32()
    );

    Ok(())
}

async fn upsert_points<T>(
    client: &Qdrant,
    embeddings: &[(Uuid, Embedding)],
    embedding_model: &Arc<T>,
) -> anyhow::Result<()>
where
    T: EmbeddingModel + Send + Sync,
{
    let mut vectors = vec![];

    for i in embeddings.rchunks(EMBED_CHUNK_SIZE) {
        let content = i
            .iter()
            .map(|(_, s)| s.content.to_string())
            .collect::<Vec<String>>();
        let mut inner_vectors = embedding_model.embed_texts(content).await?;
        vectors.append(&mut inner_vectors);
    }

    assert_eq!(vectors.len(), embeddings.len());

    let mut points = vec![];
    for (vector, embedding) in vectors.iter().zip(embeddings) {
        assert_ne!(embedding.1.path_hash.len(), 0);

        let uuid = embedding.0;

        let i = PointStruct::new(
            uuid.to_string(),
            vector.vec.iter().map(|f| *f as f32).collect::<Vec<_>>(),
            Payload::try_from(serde_json::to_value(&embedding.1)?)?,
        );

        points.push(i);
    }

    client
        .upsert_points_chunked(UpsertPointsBuilder::new(INFO_COLLECTION, points), 20)
        .await?;

    Ok(())
}

/// Load documents, split them up, and return ready embeddings.
async fn load_embeddings<T: AsRef<Path>>(
    path: T,
    blacklist: &[&str],
) -> anyhow::Result<Vec<(Uuid, Embedding)>> {
    let documents = get_documents(path, blacklist).await?;

    let mut out = vec![];
    debug!("Loaded documents");
    for (path, content) in documents {
        let content = clean_markdown_tables(&content);

        let chunks = split_text(&content);
        let chunks = merge_chunks(chunks);

        for (idx, chunk) in chunks.into_iter().enumerate() {
            let uuid = create_stable_uuid(path.to_string_lossy().trim(), idx);
            let path = path.join(format!("{idx}"));
            let path = path.to_string_lossy();
            let path_hash = Sha256::digest(path.as_bytes());
            let path_hash = format!("{:x}", path_hash);

            let content_hash = Sha256::digest(&chunk);
            let content_hash = format!("{:x}", content_hash);

            let i = Embedding {
                path_hash: path_hash.clone(),
                content: chunk,
                content_hash,
                headers: "".to_string(),
            };

            out.push((uuid, i))
        }
    }

    Ok(out)
}

fn create_stable_uuid(relative_path: &str, chunk_idx: usize) -> Uuid {
    // Combine path + index into one string
    let name = format!("{}:{}", relative_path, chunk_idx);

    // Use NAMESPACE_URL (standard) + our name → always the same UUID for same input
    Uuid::new_v5(&Uuid::NAMESPACE_URL, name.as_bytes())
}

async fn get_documents<T: AsRef<Path>>(
    path: T,
    blacklist: &[&str],
) -> anyhow::Result<Vec<(PathBuf, String)>> {
    debug!("Loading documents");
    let t = Instant::now();
    let p = path.as_ref().join("**/*.md");
    let p = p
        .to_str()
        .expect("Should work.. what idiot made this library");

    let glob = glob::glob(p).expect("Failed to read glob pattern");

    let mut handles = vec![];

    let glob = glob.filter_map(|f| f.ok());

    for ele in glob.into_iter() {
        let e = ele.to_string_lossy();

        if blacklist.iter().any(|f| e.contains(f)) {
            continue;
        }

        let future = tokio::fs::read_to_string(ele.clone());
        let handle = tokio::spawn(future);
        handles.push((handle, ele));
    }

    let mut out = vec![];
    for h in handles {
        match h.0.await {
            Ok(Ok(e)) => out.push((h.1, e)),
            e => warn!("Failed to read file: {:?}", e),
        }
    }
    debug!(
        "Loading documents took {} seconds",
        t.elapsed().as_secs_f32()
    );
    Ok(out)
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

    merge_chunks(
        chunks
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>(),
    )
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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TABLE_RE: Regex =
        Regex::new(r"(?ms)^\s*\|.*?\|\s*\n(?:\s*\|[-:]+.*?\|\s*\n)?((?:\s*\|.*?\|\s*\n)+)")
            .unwrap();
}

fn clean_markdown_tables(text: &str) -> String {
    let mut cleaned = text.to_string();

    // Replace each table with a clean, readable version
    cleaned = TABLE_RE
        .replace_all(&cleaned, |caps: &regex::Captures| {
            let table_block = caps.get(0).unwrap().as_str();

            // Simple conversion: turn table into bullet-list style text
            let lines: Vec<&str> = table_block.lines().collect();
            if lines.len() < 2 {
                return table_block.to_string();
            }

            let header = lines[0]
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|s| s.trim())
                .collect::<Vec<_>>();
            let mut result = String::new();

            // Keep header as title
            result.push_str(&format!("\n### Table: {}\n", header.join(" | ")));

            for row in lines.iter().skip(2) {
                // skip separator line
                let cells: Vec<&str> = row
                    .trim()
                    .trim_matches('|')
                    .split('|')
                    .map(|s| s.trim())
                    .collect();
                if cells.len() == header.len() {
                    let pairs: Vec<String> = header
                        .iter()
                        .zip(cells.iter())
                        .map(|(h, c)| format!("{}: {}", h, c))
                        .collect();
                    result.push_str(&format!("- {}\n", pairs.join(" | ")));
                }
            }
            result
        })
        .into_owned();

    cleaned
}

pub async fn run_update_service<T, E>(
    client: Arc<Qdrant>,
    embedding_model: Arc<E>,
    path: T,
    blacklist: &[&str],
) -> !
where
    T: AsRef<Path> + Send + Sync + 'static,
    E: EmbeddingModel + Send + Sync,
{
    let p = path.as_ref().join("**/*.md");
    let p = p
        .to_str()
        .expect("Should work.. what idiot made this library");

    let glob = glob::glob(p).expect("Failed to read glob pattern");

    let glob = glob.filter_map(|f| f.ok());

    let mut map = HashMap::new();

    for ele in glob.into_iter() {
        if let Ok(m) = ele.metadata()
            && let Ok(m) = m.modified()
        {
            map.insert(ele, m);
        }
    }

    loop {
        let mut index = false;
        let glob = glob::glob(p).expect("Failed to read glob pattern");

        let glob = glob.filter_map(|f| f.ok());
        for ele in glob.into_iter() {
            let prev_time = map.get(&ele);

            if let Some(prev) = prev_time
                && let Ok(m) = ele.metadata()
                && let Ok(time) = m.modified()
            {
                if &time > prev {
                    index = true;
                }

                map.insert(ele, time);
            } else {
                // new item

                if let Ok(m) = ele.metadata()
                    && let Ok(time) = m.modified()
                {
                    //
                    index = true;

                    map.insert(ele, time);
                }
            }
        }

        if index {
            debug!("Detected change. Indexing");
            let _ = incremental_index(client.clone(), &path, &embedding_model, blacklist).await;
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
