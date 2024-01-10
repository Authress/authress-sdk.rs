/// Client : A client configuration.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Client {
    /// The unique ID of the client.
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "createdTime")]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the client
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::ClientOptions>>,
    /// A list of the service client access keys.
    #[serde(rename = "verificationKeys", skip_serializing_if = "Option::is_none")]
    pub verification_keys: Option<Vec<crate::models::ClientAccessKey>>,
    /// The tags associated with this resource, this property is an map. { key1: value1, key2: value2 }
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<::std::collections::HashMap<String, String>>>,
}

impl Client {
    /// A client configuration.
    pub fn new(client_id: String) -> Client {
        Client {
            client_id,
            created_time: None,
            name: None,
            options: None,
            verification_keys: None,
            tags: None,
        }
    }
}


