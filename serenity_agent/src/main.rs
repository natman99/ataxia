use rig::{
    client::{Client, Nothing, builder::DefaultProviders::Ollama},
    providers::ollama::{self, OllamaExt},
};
use rustls::crypto::CryptoProvider;
use serenity_agent::MyClient;
#[tokio::main]
async fn main() {
    CryptoProvider::install_default(rustls::crypto::aws_lc_rs::default_provider())
        .expect("failed to install crypto nonsense.");
    let client = ollama::Client::new(Nothing).unwrap();
    let mut client = MyClient::new("test.json", client).await.unwrap();

    println!("Ready.");

    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let result = client.prompt(&buf).await.unwrap();

        println!("> {}", result);
    }
}
