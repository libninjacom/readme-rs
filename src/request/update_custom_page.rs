use serde_json::json;
use crate::model::*;
use crate::ReadmeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateCustomPageRequest<'a> {
    pub(crate) client: &'a ReadmeClient,
    pub slug: String,
    pub body: Option<String>,
    pub hidden: Option<bool>,
    pub html: Option<String>,
    pub htmlmode: Option<bool>,
    pub title: String,
}
impl<'a> UpdateCustomPageRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .put(&format!("/custompages/{slug}", slug = self.slug));
        if let Some(ref unwrapped) = self.body {
            r = r.push_json(json!({ "body" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.hidden {
            r = r.push_json(json!({ "hidden" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.html {
            r = r.push_json(json!({ "html" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.htmlmode {
            r = r.push_json(json!({ "htmlmode" : unwrapped }));
        }
        r = r.push_json(json!({ "title" : self.title }));
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
    pub fn html(mut self, html: &str) -> Self {
        self.html = Some(html.to_owned());
        self
    }
    pub fn htmlmode(mut self, htmlmode: bool) -> Self {
        self.htmlmode = Some(htmlmode);
        self
    }
}
