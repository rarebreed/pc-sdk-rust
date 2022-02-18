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
pub struct UserPresence {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Represents the source where the Presence was set. Some examples are: PURECLOUD, LYNC, OUTLOOK, etc.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// A boolean used to tell whether or not to set this presence source as the primary on a PATCH
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "presenceDefinition", skip_serializing_if = "Option::is_none")]
    pub presence_definition: Option<Box<crate::models::PresenceDefinition>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "modifiedDate", skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl UserPresence {
    pub fn new() -> UserPresence {
        UserPresence {
            id: None,
            name: None,
            source: None,
            primary: None,
            presence_definition: None,
            message: None,
            modified_date: None,
            self_uri: None,
        }
    }
}

