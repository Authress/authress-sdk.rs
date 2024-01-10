


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "connectionId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<Option<String>>,
    #[serde(rename = "authenticationUrl", skip_serializing_if = "Option::is_none")]
    pub authentication_url: Option<String>,
    #[serde(rename = "tokenUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub token_url: Option<Option<String>>,
    #[serde(rename = "issuerUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<Option<String>>,
    #[serde(rename = "providerCertificate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider_certificate: Option<Option<String>>,
    #[serde(rename = "clientId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<Option<String>>,
    #[serde(rename = "clientSecret", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<Option<String>>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<Box<crate::models::ConnectionData>>>,
    #[serde(rename = "defaultConnectionProperties", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_connection_properties: Option<Option<crate::models::ConnectionDefaultConnectionProperties>>,
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The tags associated with this resource, this property is an map. { key1: value1, key2: value2 }
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<::std::collections::HashMap<String, String>>>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            r#type: None,
            connection_id: None,
            authentication_url: None,
            token_url: None,
            issuer_url: None,
            provider_certificate: None,
            client_id: None,
            client_secret: None,
            data: None,
            default_connection_properties: None,
            created_time: None,
            tags: None,
        }
    }
}

/// 
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[default]
    #[serde(rename = "OAUTH2")]
    Oauth2,
    #[serde(rename = "SAML2")]
    Saml2,
    #[serde(rename = "WebAuthN")]
    WebAuthN,
}
