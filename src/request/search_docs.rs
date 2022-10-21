use serde_json::json;
use crate::model::*;
use crate::ReadmeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SearchDocsRequest<'a> {
    pub(crate) client: &'a ReadmeClient,
    pub search: String,
    pub x_readme_version: String,
}
impl<'a> SearchDocsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/docs/search");
        r = r.push_query("search", &self.search.to_string());
        r = r.header("x-readme-version", &self.x_readme_version.to_string());
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
