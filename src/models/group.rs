/// Group : A group of users, which can be added to access records.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    /// Unique identifier for the groupId, can be specified on record creation.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// A helpful name for this record
    #[serde(rename = "name")]
    pub name: String,
    /// The expected last time the group was updated
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The list of users in this group
    #[serde(rename = "users")]
    pub users: Vec<crate::models::User>,
    /// The list of admins that can edit this record even if they do not have global record edit permissions.
    #[serde(rename = "admins")]
    pub admins: Vec<crate::models::User>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Box<crate::models::Links>>>,
    /// The tags associated with this resource, this property is an map. { key1: value1, key2: value2 }
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<::std::collections::HashMap<String, String>>>,
}

impl Group {
    /// A group of users, which can be added to access records.
    pub fn new(name: String, users: Vec<crate::models::User>, admins: Vec<crate::models::User>) -> Group {
        Group {
            group_id: None,
            name,
            last_updated: None,
            users,
            admins,
            links: None,
            tags: None,
        }
    }
}


