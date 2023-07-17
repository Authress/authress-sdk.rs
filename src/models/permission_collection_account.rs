


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionCollectionAccount {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl PermissionCollectionAccount {
    pub fn new() -> PermissionCollectionAccount {
        PermissionCollectionAccount {
            account_id: None,
        }
    }
}


