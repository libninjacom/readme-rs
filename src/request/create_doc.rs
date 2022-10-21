use serde_json::json;
use crate::model::*;
use crate::ReadmeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateDocRequest<'a> {
    pub(crate) client: &'a ReadmeClient,
    pub x_readme_version: String,
    pub body: Option<String>,
    pub category: String,
    pub hidden: Option<bool>,
    pub parent_doc: Option<String>,
    pub title: String,
    pub type_: Option<String>,
}
impl<'a> CreateDocRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/docs");
        r = r.header("x-readme-version", &self.x_readme_version.to_string());
        if let Some(ref unwrapped) = self.body {
            r = r.push_json(json!({ "body" : unwrapped }));
        }
        r = r.push_json(json!({ "category" : self.category }));
        if let Some(ref unwrapped) = self.hidden {
            r = r.push_json(json!({ "hidden" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.parent_doc {
            r = r.push_json(json!({ "parentDoc" : unwrapped }));
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
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_owned());
        self
    }
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }
    pub fn parent_doc(mut self, parent_doc: &str) -> Self {
        self.parent_doc = Some(parent_doc.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
