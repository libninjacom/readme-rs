#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let version_id = "your version id";
    let response = client.get_version(version_id).send().await.unwrap();
    println!("{:#?}", response);
}
