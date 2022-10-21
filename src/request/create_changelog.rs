use serde_json::json;
use crate::model::*;
use crate::ReadmeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateChangelogRequest<'a> {
    pub(crate) client: &'a ReadmeClient,
    pub body: String,
    pub hidden: Option<bool>,
    pub title: String,
    pub type_: Option<String>,
}
impl<'a> CreateChangelogRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/changelogs");
        r = r.push_json(json!({ "body" : self.body }));
        if let Some(ref unwrapped) = self.hidden {
            r = r.push_json(json!({ "hidden" : unwrapped }));
        }
        r = r.push_json(json!({ "title" : self.title }));
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_json(json!({ "type" : unwrapped }));
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
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
