/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimUserRole : Defines a user role.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimUserRole {
    /// The role of the Genesys Cloud user.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ScimUserRole {
    /// Defines a user role.
    pub fn new() -> ScimUserRole {
        ScimUserRole {
            value: None,
        }
    }
}


