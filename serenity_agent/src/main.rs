use std::error::Error;
use std::fs;

use rig::OneOrMany;
use rig::agent::stream_to_stdout;
use rig::client::{CompletionClient, EmbeddingsClient, Nothing};
use rig::completion::{Chat, Prompt};
use rig::embeddings::EmbeddingsBuilder;
use rig::message::{AssistantContent, Message, UserContent};
use rig::providers::ollama;

use rig::streaming::StreamingChat;
use rig::tool::ToolSet;
use tokio::io;

use crate::tools::basic::{Adder, Multiply, Subtract};
use crate::tools::load_sheet::LoadSheet;

mod tools;

#[tokio::main]
async fn main() {
    let client = ollama::Client::new(Nothing).unwrap();

    let toolset = ToolSet::builder()
        .dynamic_tool(Adder)
        .dynamic_tool(Subtract)
        .dynamic_tool(Multiply)
        .dynamic_tool(LoadSheet)
        .build();

    let embedding_model = client.embedding_model(ollama::NOMIC_EMBED_TEXT);
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(toolset.schemas().unwrap())
        .unwrap()
        .build()
        .await
        .unwrap();

    let vector_store =
        rig::vector_store::in_memory_store::InMemoryVectorStore::from_documents_with_id_f(
            embeddings,
            |tool| tool.name.clone(),
        );

    let index = vector_store.index(embedding_model);

    let agent = client
        .agent("qwen3:8b")
        .preamble("You are a helpful assistant. Don't output markdown.")
        .dynamic_tools(1, index, toolset)
        .build();

    let mut messages = vec![];
    println!("Ready.");
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let m = Message::User {
            content: OneOrMany::one(UserContent::Text(rig::agent::Text { text: buf.clone() })),
        };
        messages.push(m);
        // agent.chat(prompt, chat_history)
        let response = agent.stream_chat(&buf, messages).await;

        stream_to_stdout(&mut response).await.unwrap();

        let mut m = String::new();

        use futures::StreamExt;
        use std::fmt::Write;
        use std::fmt::write;
        // while let Some(e) = response.next().await {
        //     match e {}
        // }

        buf.clear();
    }
}
