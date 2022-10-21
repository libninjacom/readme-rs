#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let title = "your title";
    let response = client
        .create_custom_page(title)
        .body("your body")
        .hidden(true)
        .html("your html")
        .htmlmode(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
