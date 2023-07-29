/// PermissionObject : The collective action and associate grants on a permission



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionObject {
    /// The action the permission grants, can be scoped using `:` and parent actions imply sub-resource permissions, action:* or action implies action:sub-action. This property is case-insensitive, it will always be cast to lowercase before comparing actions to user permissions.
    #[serde(rename = "action")]
    pub action: String,
    /// Does this permission grant the user the ability to execute the action?
    #[serde(rename = "allow")]
    pub allow: bool,
    /// Allows the user to give the permission to others without being able to execute the action.
    #[serde(rename = "grant")]
    pub grant: bool,
    /// Allows delegating or granting the permission to others without being able to execute the action.
    #[serde(rename = "delegate")]
    pub delegate: bool,
}

impl PermissionObject {
    /// The collective action and associate grants on a permission
    pub fn new(action: String, allow: bool, grant: bool, delegate: bool) -> PermissionObject {
        PermissionObject {
            action,
            allow,
            grant,
            delegate,
        }
    }
}


