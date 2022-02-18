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
pub struct AgentTimeOffRequestPatch {
    /// Whether this request has been read by the agent
    #[serde(rename = "markedAsRead", skip_serializing_if = "Option::is_none")]
    pub marked_as_read: Option<bool>,
    /// The status of this time off request. Can only be canceled if the requested date has not already passed
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Notes about the time off request. Can only be edited while the request is still pending
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl AgentTimeOffRequestPatch {
    pub fn new() -> AgentTimeOffRequestPatch {
        AgentTimeOffRequestPatch {
            marked_as_read: None,
            status: None,
            notes: None,
        }
    }
}

/// The status of this time off request. Can only be canceled if the requested date has not already passed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CANCELED")]
    CANCELED,
}

impl Default for Status {
    fn default() -> Status {
        Self::CANCELED
    }
}

