


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tenant {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "tenantLookupIdentifier", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant_lookup_identifier: Option<Option<String>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::TenantData>>,
    #[serde(rename = "connection", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connection: Option<Option<Box<crate::models::TenantConnection>>>,
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl Tenant {
    pub fn new(tenant_id: String) -> Tenant {
        Tenant {
            tenant_id,
            tenant_lookup_identifier: None,
            data: None,
            connection: None,
            created_time: None,
        }
    }
}


