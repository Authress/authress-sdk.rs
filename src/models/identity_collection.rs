


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityCollection {
    #[serde(rename = "identities")]
    pub identities: Vec<crate::models::Identity>,
}

impl IdentityCollection {
    pub fn new(identities: Vec<crate::models::Identity>) -> IdentityCollection {
        IdentityCollection {
            identities,
        }
    }
}


