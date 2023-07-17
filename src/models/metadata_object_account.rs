


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataObjectAccount {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl MetadataObjectAccount {
    pub fn new() -> MetadataObjectAccount {
        MetadataObjectAccount {
            account_id: None,
        }
    }
}


