/// ExtensionApplication : The extension's application configuration. The application contains the necessary information for users to log in to the extension.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionApplication {
    /// The unique ID of the application.
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(rename = "redirectUrls", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub redirect_urls: Option<Option<Vec<String>>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::Links>>>,
}

impl ExtensionApplication {
    /// The extension's application configuration. The application contains the necessary information for users to log in to the extension.
    pub fn new(application_id: String) -> ExtensionApplication {
        ExtensionApplication {
            application_id,
            redirect_urls: None,
            links: None,
        }
    }
}


