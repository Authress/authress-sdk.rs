/// ExtensionApplication : The extension's application configuration. The application contains the necessary information for users to log in to the extension.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionApplication {
    /// The unique ID of the application.
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(rename = "redirectUrls", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub redirect_urls: Option<Option<Vec<String>>>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::Links>,
}

impl ExtensionApplication {
    /// The extension's application configuration. The application contains the necessary information for users to log in to the extension.
    pub fn new(application_id: String, links: crate::models::Links) -> ExtensionApplication {
        ExtensionApplication {
            application_id,
            redirect_urls: None,
            links: Box::new(links),
        }
    }
}


