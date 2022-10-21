#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let id = "your id";
    let response = client.delete_api_specification(id).send().await.unwrap();
    println!("{:#?}", response);
}
