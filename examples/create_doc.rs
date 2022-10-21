#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let x_readme_version = "your x readme version";
    let category = "your category";
    let title = "your title";
    let response = client
        .create_doc(x_readme_version, category, title)
        .body("your body")
        .hidden(true)
        .parent_doc("your parent doc")
        .type_("your type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
