use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Changelog {
    #[serde(rename = "body")]
    ///Body content of the changelog
    pub body: String,
    #[serde(rename = "hidden")]
    ///Visibility of the changelog
    pub hidden: Option<bool>,
    #[serde(rename = "title")]
    ///Title of the changelog
    pub title: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for Changelog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomPage {
    #[serde(rename = "body")]
    ///Body formatted in Markdown (displayed by default).
    pub body: Option<String>,
    #[serde(rename = "hidden")]
    ///Visibility of the custom page
    pub hidden: Option<bool>,
    #[serde(rename = "html")]
    ///Body formatted in HTML (sanitized, only displayed if `htmlmode` is **true**).
    pub html: Option<String>,
    #[serde(rename = "htmlmode")]
    ///**true** if `html` should be displayed, **false** if `body` should be displayed.
    pub htmlmode: Option<bool>,
    #[serde(rename = "title")]
    ///Title of the custom page
    pub title: String,
}
impl std::fmt::Display for CustomPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    #[serde(rename = "body")]
    ///Body content of the page, formatted in ReadMe or Github flavored Markdown. Accepts long page content, for example, greater than 100k characters
    pub body: Option<String>,
    #[serde(rename = "category")]
    ///Category ID of the page, which you can get through https://docs.readme.com/developers/reference/categories#getcategory
    pub category: String,
    #[serde(rename = "hidden")]
    ///Visibility of the page
    pub hidden: Option<bool>,
    #[serde(rename = "parentDoc")]
    ///For a subpage, specify the parent doc ID, which you can get through https://docs.readme.com/developers/reference/docs#getdoc
    pub parent_doc: Option<String>,
    #[serde(rename = "title")]
    ///Title of the page
    pub title: String,
    #[serde(rename = "type")]
    ///Type of the page. The available types all show up under the /docs/ URL path of your docs project (also known as the "guides" section). Can be "basic" (most common), "error" (page desribing an API error), or "link" (page that redirects to an external link)
    pub type_: Option<String>,
}
impl std::fmt::Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "codename")]
    ///Dubbed name of version
    pub codename: Option<String>,
    #[serde(rename = "from")]
    ///Semantic Version to use as the base fork
    pub from: String,
    #[serde(rename = "is_beta")]
    pub is_beta: Option<bool>,
    #[serde(rename = "is_deprecated")]
    ///Should this be deprecated? Only allowed in PUT operations
    pub is_deprecated: Option<bool>,
    #[serde(rename = "is_hidden")]
    ///Should this be publically accessible?
    pub is_hidden: Option<bool>,
    #[serde(rename = "is_stable")]
    ///Should this be the **main** version
    pub is_stable: Option<bool>,
    #[serde(rename = "version")]
    ///Semantic Version
    pub version: String,
}
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
