use serde_json::json;
use crate::model::*;
use crate::ReadmeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateVersionRequest<'a> {
    pub(crate) client: &'a ReadmeClient,
    pub codename: Option<String>,
    pub from: String,
    pub is_beta: Option<bool>,
    pub is_deprecated: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_stable: Option<bool>,
    pub version: String,
}
impl<'a> CreateVersionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/version");
        if let Some(ref unwrapped) = self.codename {
            r = r.push_json(json!({ "codename" : unwrapped }));
        }
        r = r.push_json(json!({ "from" : self.from }));
        if let Some(ref unwrapped) = self.is_beta {
            r = r.push_json(json!({ "is_beta" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_deprecated {
            r = r.push_json(json!({ "is_deprecated" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_hidden {
            r = r.push_json(json!({ "is_hidden" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_stable {
            r = r.push_json(json!({ "is_stable" : unwrapped }));
        }
        r = r.push_json(json!({ "version" : self.version }));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn codename(mut self, codename: &str) -> Self {
        self.codename = Some(codename.to_owned());
        self
    }
    pub fn is_beta(mut self, is_beta: bool) -> Self {
        self.is_beta = Some(is_beta);
        self
    }
    pub fn is_deprecated(mut self, is_deprecated: bool) -> Self {
        self.is_deprecated = Some(is_deprecated);
        self
    }
    pub fn is_hidden(mut self, is_hidden: bool) -> Self {
        self.is_hidden = Some(is_hidden);
        self
    }
    pub fn is_stable(mut self, is_stable: bool) -> Self {
        self.is_stable = Some(is_stable);
        self
    }
}
