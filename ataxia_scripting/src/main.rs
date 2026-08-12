use std::fs;

use url::Url;

#[tokio::main]
async fn main() {
    // rhai::CustomType;
    //
    //
    env_logger::init();

    let z = ataxia_scripting::registry::scrape_websites("silvery barbs")
        .await
        .unwrap();
    let r = ataxia_scripting::registry::ask_llm(
        z,
        Url::parse("http://localhost:8080/v1").unwrap(),
        None,
        "Qwen3-8B-Q8_0",
    )
    .await
    .unwrap();
    println!("{r:?}");
}
