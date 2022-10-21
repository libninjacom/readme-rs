#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let response = client.get_versions().send().await.unwrap();
    println!("{:#?}", response);
}
