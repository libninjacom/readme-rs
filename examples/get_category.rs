#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let slug = "your slug";
    let x_readme_version = "your x readme version";
    let response = client.get_category(slug, x_readme_version).send().await.unwrap();
    println!("{:#?}", response);
}
