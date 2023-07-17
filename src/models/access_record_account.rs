


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessRecordAccount {
    #[serde(rename = "accountId")]
    pub account_id: String,
}

impl AccessRecordAccount {
    pub fn new(account_id: String) -> AccessRecordAccount {
        AccessRecordAccount {
            account_id,
        }
    }
}


