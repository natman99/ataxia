use std::io::Write;

use log::info;
use rig::{
    client::{CompletionClient, Nothing},
    providers::ollama::{self, EmbeddingModel},
};
use rustls::crypto::CryptoProvider;
use serenity_agent::MyClient;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();
    CryptoProvider::install_default(rustls::crypto::aws_lc_rs::default_provider())
        .expect("failed to install crypto nonsense.");
    let client = ollama::Client::new(Nothing).unwrap();
    let mut client = MyClient::new("test.json", client).await.unwrap();

    info!("Ready.");

    loop {
        let mut buf = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buf).unwrap();
        let result = client.prompt(&buf).await.unwrap();

        println!(">> {}", result);
    }
}
