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
pub struct AnalyticsUserPresenceRecord {
    /// The start time of the record. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The end time of the record. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The user's system presence
    #[serde(rename = "systemPresence", skip_serializing_if = "Option::is_none")]
    pub system_presence: Option<SystemPresence>,
    /// The identifier for the user's organization presence
    #[serde(rename = "organizationPresenceId", skip_serializing_if = "Option::is_none")]
    pub organization_presence_id: Option<String>,
}

impl AnalyticsUserPresenceRecord {
    pub fn new() -> AnalyticsUserPresenceRecord {
        AnalyticsUserPresenceRecord {
            start_time: None,
            end_time: None,
            system_presence: None,
            organization_presence_id: None,
        }
    }
}

/// The user's system presence
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemPresence {
    #[serde(rename = "AVAILABLE")]
    AVAILABLE,
    #[serde(rename = "AWAY")]
    AWAY,
    #[serde(rename = "BUSY")]
    BUSY,
    #[serde(rename = "OFFLINE")]
    OFFLINE,
    #[serde(rename = "IDLE")]
    IDLE,
    #[serde(rename = "ON_QUEUE")]
    ONQUEUE,
    #[serde(rename = "MEAL")]
    MEAL,
    #[serde(rename = "TRAINING")]
    TRAINING,
    #[serde(rename = "MEETING")]
    MEETING,
    #[serde(rename = "BREAK")]
    _BREAK,
}

impl Default for SystemPresence {
    fn default() -> SystemPresence {
        Self::AVAILABLE
    }
}

