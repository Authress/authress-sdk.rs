


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantData {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
}

impl TenantData {
    pub fn new() -> TenantData {
        TenantData {
            name: None,
        }
    }
}


