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
pub struct UserAuthorization {
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::models::DomainRole>>,
    /// A collection of the roles the user is not using
    #[serde(rename = "unusedRoles", skip_serializing_if = "Option::is_none")]
    pub unused_roles: Option<Vec<crate::models::DomainRole>>,
    /// A collection of the permissions granted by all assigned roles
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// The policies configured for assigned permissions.
    #[serde(rename = "permissionPolicies", skip_serializing_if = "Option::is_none")]
    pub permission_policies: Option<Vec<crate::models::ResourcePermissionPolicy>>,
}

impl UserAuthorization {
    pub fn new() -> UserAuthorization {
        UserAuthorization {
            roles: None,
            unused_roles: None,
            permissions: None,
            permission_policies: None,
        }
    }
}

