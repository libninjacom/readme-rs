#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let response = client.upload_swagger().send().await.unwrap();
    println!("{:#?}", response);
}
