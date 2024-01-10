


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "createdTime")]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::Links>>>,
}

impl Account {
    pub fn new(account_id: String) -> Account {
        Account {
            account_id,
            created_time: None,
            name: None,
            links: None,
        }
    }
}


