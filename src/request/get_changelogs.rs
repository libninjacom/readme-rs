use serde_json::json;
use crate::model::*;
use crate::ReadmeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetChangelogsRequest<'a> {
    pub(crate) client: &'a ReadmeClient,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
}
impl<'a> GetChangelogsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.get("/changelogs");
        if let Some(ref unwrapped) = self.per_page {
            r = r.push_query("perPage", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn per_page(mut self, per_page: i64) -> Self {
        self.per_page = Some(per_page);
        self
    }
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
}
