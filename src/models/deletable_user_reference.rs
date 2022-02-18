/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// DeletableUserReference : User reference with delete flag to remove the user from an associated entity



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeletableUserReference {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If marked true, the user will be removed an associated entity
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl DeletableUserReference {
    /// User reference with delete flag to remove the user from an associated entity
    pub fn new() -> DeletableUserReference {
        DeletableUserReference {
            id: None,
            delete: None,
            self_uri: None,
        }
    }
}

