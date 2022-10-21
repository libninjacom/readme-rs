use serde_json::json;
use crate::model::*;
use crate::ReadmeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UploadApiSpecificationRequest<'a> {
    pub(crate) client: &'a ReadmeClient,
    pub x_readme_version: String,
}
impl<'a> UploadApiSpecificationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/api-specification");
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
