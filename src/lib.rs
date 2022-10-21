//! [`ReadmeClient`](struct.ReadmeClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct ReadmeClient {
    pub(crate) client: httpclient::Client,
    authentication: ReadmeAuthentication,
}
impl ReadmeClient {
    pub fn from_env() -> Self {
        let url = "https://dash.readme.io/api/v1".to_string();
        Self {
            client: httpclient::Client::new(Some(url)),
            authentication: ReadmeAuthentication::from_env(),
        }
    }
}
impl ReadmeClient {
    pub fn new(url: &str, authentication: ReadmeAuthentication) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: ReadmeAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {}
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**Get metadata about the current project

Returns project data for API key*/
    pub fn get_project(&self) -> request::GetProjectRequest {
        request::GetProjectRequest {
            client: &self,
        }
    }
    ///Get API specification metadata
    pub fn get_api_specification(
        &self,
        x_readme_version: &str,
    ) -> request::GetApiSpecificationRequest {
        request::GetApiSpecificationRequest {
            client: &self,
            per_page: None,
            page: None,
            x_readme_version: x_readme_version.to_owned(),
        }
    }
    ///Upload an API specification to ReadMe. Or, to use a newer solution see https://docs.readme.com/guides/docs/automatically-sync-api-specification-with-github
    pub fn upload_api_specification(
        &self,
        x_readme_version: &str,
    ) -> request::UploadApiSpecificationRequest {
        request::UploadApiSpecificationRequest {
            client: &self,
            x_readme_version: x_readme_version.to_owned(),
        }
    }
    ///Update an API specification in ReadMe
    pub fn update_api_specification(
        &self,
        id: &str,
    ) -> request::UpdateApiSpecificationRequest {
        request::UpdateApiSpecificationRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    ///Delete an API specification in ReadMe
    pub fn delete_api_specification(
        &self,
        id: &str,
    ) -> request::DeleteApiSpecificationRequest {
        request::DeleteApiSpecificationRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Get category

Returns the category with this slug*/
    pub fn get_category(
        &self,
        slug: &str,
        x_readme_version: &str,
    ) -> request::GetCategoryRequest {
        request::GetCategoryRequest {
            client: &self,
            slug: slug.to_owned(),
            x_readme_version: x_readme_version.to_owned(),
        }
    }
    /**Get docs for category

Returns the docs and children docs within this category*/
    pub fn get_category_docs(
        &self,
        slug: &str,
        x_readme_version: &str,
    ) -> request::GetCategoryDocsRequest {
        request::GetCategoryDocsRequest {
            client: &self,
            slug: slug.to_owned(),
            x_readme_version: x_readme_version.to_owned(),
        }
    }
    /**Get changelogs

Returns a list of changelogs associated with the project API key*/
    pub fn get_changelogs(&self) -> request::GetChangelogsRequest {
        request::GetChangelogsRequest {
            client: &self,
            per_page: None,
            page: None,
        }
    }
    /**Create changelog

Create a new changelog inside of this project*/
    pub fn create_changelog(
        &self,
        body: &str,
        title: &str,
    ) -> request::CreateChangelogRequest {
        request::CreateChangelogRequest {
            client: &self,
            body: body.to_owned(),
            hidden: None,
            title: title.to_owned(),
            type_: None,
        }
    }
    /**Get changelog

Returns the changelog with this slug*/
    pub fn get_changelog(&self, slug: &str) -> request::GetChangelogRequest {
        request::GetChangelogRequest {
            client: &self,
            slug: slug.to_owned(),
        }
    }
    /**Update changelog

Update a changelog with this slug*/
    pub fn update_changelog(
        &self,
        slug: &str,
        body: &str,
        title: &str,
    ) -> request::UpdateChangelogRequest {
        request::UpdateChangelogRequest {
            client: &self,
            slug: slug.to_owned(),
            body: body.to_owned(),
            hidden: None,
            title: title.to_owned(),
            type_: None,
        }
    }
    /**Delete changelog

Delete the changelog with this slug*/
    pub fn delete_changelog(&self, slug: &str) -> request::DeleteChangelogRequest {
        request::DeleteChangelogRequest {
            client: &self,
            slug: slug.to_owned(),
        }
    }
    /**Get custom pages

Returns a list of custom pages associated with the project API key*/
    pub fn get_custom_pages(&self) -> request::GetCustomPagesRequest {
        request::GetCustomPagesRequest {
            client: &self,
            per_page: None,
            page: None,
        }
    }
    /**Create custom page

Create a new custom page inside of this project*/
    pub fn create_custom_page(&self, title: &str) -> request::CreateCustomPageRequest {
        request::CreateCustomPageRequest {
            client: &self,
            body: None,
            hidden: None,
            html: None,
            htmlmode: None,
            title: title.to_owned(),
        }
    }
    /**Get custom page

Returns the custom page with this slug*/
    pub fn get_custom_page(&self, slug: &str) -> request::GetCustomPageRequest {
        request::GetCustomPageRequest {
            client: &self,
            slug: slug.to_owned(),
        }
    }
    /**Update custom page

Update a custom page with this slug*/
    pub fn update_custom_page(
        &self,
        slug: &str,
        title: &str,
    ) -> request::UpdateCustomPageRequest {
        request::UpdateCustomPageRequest {
            client: &self,
            slug: slug.to_owned(),
            body: None,
            hidden: None,
            html: None,
            htmlmode: None,
            title: title.to_owned(),
        }
    }
    /**Delete custom page

Delete the custom page with this slug*/
    pub fn delete_custom_page(&self, slug: &str) -> request::DeleteCustomPageRequest {
        request::DeleteCustomPageRequest {
            client: &self,
            slug: slug.to_owned(),
        }
    }
    /**Create doc

Create a new doc inside of this project*/
    pub fn create_doc(
        &self,
        x_readme_version: &str,
        category: &str,
        title: &str,
    ) -> request::CreateDocRequest {
        request::CreateDocRequest {
            client: &self,
            x_readme_version: x_readme_version.to_owned(),
            body: None,
            category: category.to_owned(),
            hidden: None,
            parent_doc: None,
            title: title.to_owned(),
            type_: None,
        }
    }
    /**Search docs

Returns all docs that match the search*/
    pub fn search_docs(
        &self,
        search: &str,
        x_readme_version: &str,
    ) -> request::SearchDocsRequest {
        request::SearchDocsRequest {
            client: &self,
            search: search.to_owned(),
            x_readme_version: x_readme_version.to_owned(),
        }
    }
    /**Get doc

Returns the doc with this slug*/
    pub fn get_doc(&self, slug: &str, x_readme_version: &str) -> request::GetDocRequest {
        request::GetDocRequest {
            client: &self,
            slug: slug.to_owned(),
            x_readme_version: x_readme_version.to_owned(),
        }
    }
    /**Update doc

Update a doc with this slug*/
    pub fn update_doc(
        &self,
        args: request::UpdateDocRequired,
    ) -> request::UpdateDocRequest {
        request::UpdateDocRequest {
            client: &self,
            slug: args.slug.to_owned(),
            x_readme_version: args.x_readme_version.to_owned(),
            body: None,
            category: args.category.to_owned(),
            hidden: None,
            parent_doc: None,
            title: args.title.to_owned(),
            type_: None,
        }
    }
    /**Delete doc

Delete the doc with this slug*/
    pub fn delete_doc(
        &self,
        slug: &str,
        x_readme_version: &str,
    ) -> request::DeleteDocRequest {
        request::DeleteDocRequest {
            client: &self,
            slug: slug.to_owned(),
            x_readme_version: x_readme_version.to_owned(),
        }
    }
    /**Get errors

Returns with all of the error page types for this project*/
    pub fn get_errors(&self) -> request::GetErrorsRequest {
        request::GetErrorsRequest {
            client: &self,
        }
    }
    ///DEPRECATED. Instead use https://docs.readme.com/developers/reference/api-specification#uploadapispecification to upload a Swagger file to ReadMe
    pub fn upload_swagger(&self) -> request::UploadSwaggerRequest {
        request::UploadSwaggerRequest {
            client: &self,
        }
    }
    ///DEPRECATED. Instead, use https://docs.readme.com/developers/reference/api-specification#updateapispecification to update a Swagger file.
    pub fn update_swagger(&self, id: &str) -> request::UpdateSwaggerRequest {
        request::UpdateSwaggerRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    ///DEPRECATED. Instead, use https://docs.readme.com/developers/reference/api-specification#deleteapispecification to delete a Swagger file in ReadMe
    pub fn delete_swagger(&self, id: &str) -> request::DeleteSwaggerRequest {
        request::DeleteSwaggerRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    /**Get versions

Retrieve a list of versions associated with a project API key*/
    pub fn get_versions(&self) -> request::GetVersionsRequest {
        request::GetVersionsRequest {
            client: &self,
        }
    }
    /**Create version

Create a new version*/
    pub fn create_version(
        &self,
        from: &str,
        version: &str,
    ) -> request::CreateVersionRequest {
        request::CreateVersionRequest {
            client: &self,
            codename: None,
            from: from.to_owned(),
            is_beta: None,
            is_deprecated: None,
            is_hidden: None,
            is_stable: None,
            version: version.to_owned(),
        }
    }
    /**Get version

Returns the version with this version ID*/
    pub fn get_version(&self, version_id: &str) -> request::GetVersionRequest {
        request::GetVersionRequest {
            client: &self,
            version_id: version_id.to_owned(),
        }
    }
    /**Update version

Update an existing version*/
    pub fn update_version(
        &self,
        version_id: &str,
        from: &str,
        version: &str,
    ) -> request::UpdateVersionRequest {
        request::UpdateVersionRequest {
            client: &self,
            version_id: version_id.to_owned(),
            codename: None,
            from: from.to_owned(),
            is_beta: None,
            is_deprecated: None,
            is_hidden: None,
            is_stable: None,
            version: version.to_owned(),
        }
    }
    /**Delete version

Delete a version*/
    pub fn delete_version(&self, version_id: &str) -> request::DeleteVersionRequest {
        request::DeleteVersionRequest {
            client: &self,
            version_id: version_id.to_owned(),
        }
    }
}
