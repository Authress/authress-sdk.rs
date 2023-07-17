/// AccessRequest : The access requested by a user.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessRequest {
    /// Unique identifier for the request.
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// The expected last time the request was updated
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// Current status of the access request.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "access")]
    pub access: Box<crate::models::AccessTemplate>,
    #[serde(rename = "links")]
    pub links: Box<crate::models::AccountLinks>,
    /// The tags associated with this resource, this property is an map. { key1: value1, key2: value2 }
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<::std::collections::HashMap<String, String>>>,
}

impl AccessRequest {
    /// The access requested by a user.
    pub fn new(request_id: String, access: crate::models::AccessTemplate, links: crate::models::AccountLinks) -> AccessRequest {
        AccessRequest {
            request_id,
            last_updated: None,
            status: None,
            access: Box::new(access),
            links: Box::new(links),
            tags: None,
        }
    }
}

/// Current status of the access request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "DENIED")]
    Denied,
    #[serde(rename = "DELETED")]
    Deleted,
}

impl Default for Status {
    fn default() -> Status {
        Self::Open
    }
}

