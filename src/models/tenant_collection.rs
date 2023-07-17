/// TenantCollection : A collection of tenants.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantCollection {
    #[serde(rename = "tenants")]
    pub tenants: Vec<crate::models::Tenant>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
}

impl TenantCollection {
    /// A collection of tenants.
    pub fn new(tenants: Vec<crate::models::Tenant>) -> TenantCollection {
        TenantCollection {
            tenants,
            pagination: None,
        }
    }
}


