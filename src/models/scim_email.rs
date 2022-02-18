/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimEmail : Defines a SCIM email address.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimEmail {
    /// The email address. Is immutable if \"type\" is set to \"other\".
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The type of email address. \"value\" is immutable if \"type\" is set to \"other\".
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Indicates whether the email address is the primary email address.
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

impl ScimEmail {
    /// Defines a SCIM email address.
    pub fn new() -> ScimEmail {
        ScimEmail {
            value: None,
            _type: None,
            primary: None,
        }
    }
}

/// The type of email address. \"value\" is immutable if \"type\" is set to \"other\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Work
    }
}
