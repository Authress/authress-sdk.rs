


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountCollection {
    #[serde(rename = "accounts")]
    pub accounts: Box<crate::models::Account>,
}

impl AccountCollection {
    pub fn new(accounts: crate::models::Account) -> AccountCollection {
        AccountCollection {
            accounts: Box::new(accounts),
        }
    }
}


