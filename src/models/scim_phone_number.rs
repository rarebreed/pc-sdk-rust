/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimPhoneNumber : Defines a SCIM phone number.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimPhoneNumber {
    /// The phone number in E.164 or tel URI format, for example, tel:+nnnnnnnn; ext=xxxxx.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The type of phone number.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Indicates whether the phone number is the primary phone number.
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

impl ScimPhoneNumber {
    /// Defines a SCIM phone number.
    pub fn new() -> ScimPhoneNumber {
        ScimPhoneNumber {
            value: None,
            _type: None,
            primary: None,
        }
    }
}

/// The type of phone number.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "work2")]
    Work2,
    #[serde(rename = "work3")]
    Work3,
    #[serde(rename = "work4")]
    Work4,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "microsoftteams")]
    Microsoftteams,
    #[serde(rename = "zoomphone")]
    Zoomphone,
    #[serde(rename = "ringcentral")]
    Ringcentral,
}

impl Default for Type {
    fn default() -> Type {
        Self::Work
    }
}
