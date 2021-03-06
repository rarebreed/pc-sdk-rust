/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionDetails {
    /// The type of permission requirement
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// List of required permissions
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// Whether the current user can subscribe, when division permissions are otherwise required
    #[serde(rename = "allowsCurrentUser", skip_serializing_if = "Option::is_none")]
    pub allows_current_user: Option<bool>,
    /// Whether or not this permission requirement is enforced
    #[serde(rename = "enforced", skip_serializing_if = "Option::is_none")]
    pub enforced: Option<bool>,
}

impl PermissionDetails {
    pub fn new() -> PermissionDetails {
        PermissionDetails {
            _type: None,
            permissions: None,
            allows_current_user: None,
            enforced: None,
        }
    }
}

/// The type of permission requirement
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "requiresCurrentUser")]
    RequiresCurrentUser,
    #[serde(rename = "requiresPermissions")]
    RequiresPermissions,
    #[serde(rename = "requiresDivisionPermissions")]
    RequiresDivisionPermissions,
    #[serde(rename = "requiresAnyDivisionPermissions")]
    RequiresAnyDivisionPermissions,
    #[serde(rename = "requiresUserBeConversationParticipant")]
    RequiresUserBeConversationParticipant,
}

impl Default for Type {
    fn default() -> Type {
        Self::RequiresCurrentUser
    }
}

