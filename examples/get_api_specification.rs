#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let x_readme_version = "your x readme version";
    let response = client
        .get_api_specification(x_readme_version)
        .per_page(1)
        .page(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
