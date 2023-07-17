/// MetadataObject : Metadata and additional properties relevant.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataObject {
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<crate::models::MetadataObjectAccount>>,
    #[serde(rename = "userId")]
    pub user_id: crate::models::UserId,
    /// A JSON object limited to 10KB. The owner identified by the sub will always have access to read and update this data. Service clients may have access if the related property on the client is set. Access is restricted to authorized users.
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<serde_json::Value>,
}

impl MetadataObject {
    /// Metadata and additional properties relevant.
    pub fn new(user_id: crate::models::UserId, metadata: Option<serde_json::Value>) -> MetadataObject {
        MetadataObject {
            account: None,
            user_id,
            metadata,
        }
    }
}


