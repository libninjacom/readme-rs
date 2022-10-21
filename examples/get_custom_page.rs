#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let slug = "your slug";
    let response = client.get_custom_page(slug).send().await.unwrap();
    println!("{:#?}", response);
}
