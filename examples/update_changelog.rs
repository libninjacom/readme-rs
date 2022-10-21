#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let slug = "your slug";
    let body = "your body";
    let title = "your title";
    let response = client
        .update_changelog(slug, body, title)
        .hidden(true)
        .type_("your type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
