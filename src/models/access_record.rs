/// AccessRecord : The access record which links users to roles.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessRecord {
    /// Unique identifier for the record, can be specified on record creation.
    #[serde(rename = "recordId", skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// A helpful name for this record
    #[serde(rename = "name")]
    pub name: String,
    /// More details about this record
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Percentage capacity of record that is filled.
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f32>,
    /// The expected last time the record was updated
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// Current status of the access record.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The list of users this record applies to
    #[serde(rename = "users", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub users: Option<Option<Vec<crate::models::User>>>,
    /// The list of admin that can edit this record even if they do not have global record edit permissions.
    #[serde(rename = "admins", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub admins: Option<Option<Vec<crate::models::User>>>,
    /// The list of groups this record applies to. Users in these groups will be receive access to the resources listed.
    #[serde(rename = "groups", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Option<Vec<crate::models::LinkedGroup>>>,
    /// A list of statements which match roles to resources.
    #[serde(rename = "statements")]
    pub statements: Vec<crate::models::Statement>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::Links>>>,
    /// The tags associated with this resource, this property is an map. { key1: value1, key2: value2 }
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<::std::collections::HashMap<String, String>>>,
}

impl AccessRecord {
    /// The access record which links users to roles.
    pub fn new(name: String, statements: Vec<crate::models::Statement>) -> AccessRecord {
        AccessRecord {
            record_id: None,
            name,
            description: None,
            capacity: None,
            last_updated: None,
            status: None,
            users: None,
            admins: None,
            groups: None,
            statements,
            links: None,
            tags: None,
        }
    }
}

/// Current status of the access record.
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[default]
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DELETED")]
    Deleted,
}