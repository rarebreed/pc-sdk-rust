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
pub struct RoutingStatus {
    /// The userId of the agent
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Indicates the Routing State of the agent.  A value of OFF_QUEUE will be returned if the specified user does not exist.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The timestamp when the agent went into this state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl RoutingStatus {
    pub fn new() -> RoutingStatus {
        RoutingStatus {
            user_id: None,
            status: None,
            start_time: None,
        }
    }
}

/// Indicates the Routing State of the agent.  A value of OFF_QUEUE will be returned if the specified user does not exist.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "OFF_QUEUE")]
    OFFQUEUE,
    #[serde(rename = "IDLE")]
    IDLE,
    #[serde(rename = "INTERACTING")]
    INTERACTING,
    #[serde(rename = "NOT_RESPONDING")]
    NOTRESPONDING,
    #[serde(rename = "COMMUNICATING")]
    COMMUNICATING,
}

impl Default for Status {
    fn default() -> Status {
        Self::OFFQUEUE
    }
}

