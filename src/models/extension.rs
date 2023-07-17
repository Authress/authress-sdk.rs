


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    #[serde(rename = "extensionId")]
    pub extension_id: String,
    /// The name of the extension. This name is visible in the Authress management portal
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<crate::models::ExtensionApplication>>,
    #[serde(rename = "client")]
    pub client: Box<crate::models::ExtensionClient>,
    /// The tags associated with this resource, this property is an map. { key1: value1, key2: value2 }
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<::std::collections::HashMap<String, String>>>,
}

impl Extension {
    pub fn new(extension_id: String, created_time: String, client: crate::models::ExtensionClient) -> Extension {
        Extension {
            extension_id,
            name: None,
            created_time,
            application: None,
            client: Box::new(client),
            tags: None,
        }
    }
}


