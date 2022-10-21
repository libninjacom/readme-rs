use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateDocRequired {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub x_readme_version: String,
}
impl std::fmt::Display for UpdateDocRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Version {
    ///Dubbed name of version
    pub codename: Option<String>,
    ///Semantic Version to use as the base fork
    pub from: String,
    ///Should this be publically accessible?
    pub is_hidden: Option<bool>,
    ///Should this be the **main** version
    pub is_stable: Option<bool>,
    ///Should this be deprecated? Only allowed in PUT operations
    pub is_deprecated: Option<bool>,
    ///Semantic Version
    pub version: String,
    pub is_beta: Option<bool>,
}
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Changelog {
    pub type_: Option<String>,
    ///Visibility of the changelog
    pub hidden: Option<bool>,
    ///Title of the changelog
    pub title: String,
    ///Body content of the changelog
    pub body: String,
}
impl std::fmt::Display for Changelog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Doc {
    ///Body content of the page, formatted in ReadMe or Github flavored Markdown. Accepts long page content, for example, greater than 100k characters
    pub body: Option<String>,
    ///Category ID of the page, which you can get through https://docs.readme.com/developers/reference/categories#getcategory
    pub category: String,
    ///Visibility of the page
    pub hidden: Option<bool>,
    ///For a subpage, specify the parent doc ID, which you can get through https://docs.readme.com/developers/reference/docs#getdoc
    pub parent_doc: Option<String>,
    ///Title of the page
    pub title: String,
    ///Type of the page. The available types all show up under the /docs/ URL path of your docs project (also known as the "guides" section). Can be "basic" (most common), "error" (page desribing an API error), or "link" (page that redirects to an external link)
    pub type_: Option<String>,
}
impl std::fmt::Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomPage {
    ///Body formatted in Markdown (displayed by default).
    pub body: Option<String>,
    ///Title of the custom page
    pub title: String,
    ///**true** if `html` should be displayed, **false** if `body` should be displayed.
    pub htmlmode: Option<bool>,
    ///Visibility of the custom page
    pub hidden: Option<bool>,
    ///Body formatted in HTML (sanitized, only displayed if `htmlmode` is **true**).
    pub html: Option<String>,
}
impl std::fmt::Display for CustomPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
