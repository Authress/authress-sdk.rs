/// LinkedGroup : The referenced group



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedGroup {
    /// The unique identifier for the group.
    #[serde(rename = "groupId")]
    pub group_id: String,
}

impl LinkedGroup {
    /// The referenced group
    pub fn new(group_id: String) -> LinkedGroup {
        LinkedGroup {
            group_id,
        }
    }
}


