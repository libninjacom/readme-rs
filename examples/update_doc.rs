#![allow(unused_imports)]
use readme_api::ReadmeClient;
use readme_api::model::*;
use readme_api::request::UpdateDocRequired;
#[tokio::main]
async fn main() {
    let client = ReadmeClient::from_env();
    let args = UpdateDocRequired {
        slug: "your slug",
        title: "your title",
        category: "your category",
        x_readme_version: "your x readme version",
    };
    let response = client
        .update_doc(args)
        .body("your body")
        .hidden(true)
        .parent_doc("your parent doc")
        .type_("your type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
