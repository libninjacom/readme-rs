#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let from = "your from";
    let version = "your version";
    let response = client
        .create_version(from, version)
        .codename("your codename")
        .is_beta(true)
        .is_deprecated(true)
        .is_hidden(true)
        .is_stable(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
