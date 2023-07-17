


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionData {
    #[serde(rename = "tenantId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "supportedContentType", skip_serializing_if = "Option::is_none")]
    pub supported_content_type: Option<SupportedContentType>,
}

impl ConnectionData {
    pub fn new() -> ConnectionData {
        ConnectionData {
            tenant_id: None,
            name: None,
            supported_content_type: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedContentType {
    #[serde(rename = "application/json")]
    Json,
    #[serde(rename = "application/x-www-form-urlencoded")]
    XWwwFormUrlencoded,
}

impl Default for SupportedContentType {
    fn default() -> SupportedContentType {
        Self::Json
    }
}

