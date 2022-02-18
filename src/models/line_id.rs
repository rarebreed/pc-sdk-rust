/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// LineId : User information for a Line account



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LineId {
    /// The set of Line userIds that this person has. Each userId is specific to the Line channel that the user interacts with.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<crate::models::LineUserId>>,
    /// The displayName of this person's account in Line
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl LineId {
    /// User information for a Line account
    pub fn new() -> LineId {
        LineId {
            ids: None,
            display_name: None,
        }
    }
}


