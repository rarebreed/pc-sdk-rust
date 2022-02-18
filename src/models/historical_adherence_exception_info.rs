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
pub struct HistoricalAdherenceExceptionInfo {
    /// Exception start offset in seconds relative to query start time
    #[serde(rename = "startOffsetSeconds", skip_serializing_if = "Option::is_none")]
    pub start_offset_seconds: Option<i32>,
    /// Exception end offset in seconds relative to query start time
    #[serde(rename = "endOffsetSeconds", skip_serializing_if = "Option::is_none")]
    pub end_offset_seconds: Option<i32>,
    /// The ID of the scheduled activity code for this user
    #[serde(rename = "scheduledActivityCodeId", skip_serializing_if = "Option::is_none")]
    pub scheduled_activity_code_id: Option<String>,
    /// Activity for which the user is scheduled
    #[serde(rename = "scheduledActivityCategory", skip_serializing_if = "Option::is_none")]
    pub scheduled_activity_category: Option<ScheduledActivityCategory>,
    /// Activity for which the user is actually engaged
    #[serde(rename = "actualActivityCategory", skip_serializing_if = "Option::is_none")]
    pub actual_activity_category: Option<ActualActivityCategory>,
    /// Actual underlying system presence value
    #[serde(rename = "systemPresence", skip_serializing_if = "Option::is_none")]
    pub system_presence: Option<SystemPresence>,
    #[serde(rename = "routingStatus", skip_serializing_if = "Option::is_none")]
    pub routing_status: Option<Box<crate::models::RoutingStatus>>,
    /// The impact of the current adherence state for this user
    #[serde(rename = "impact", skip_serializing_if = "Option::is_none")]
    pub impact: Option<Impact>,
    /// The lookup ID used to retrieve the actual secondary status from map of lookup ID to corresponding secondary presence ID
    #[serde(rename = "secondaryPresenceLookupId", skip_serializing_if = "Option::is_none")]
    pub secondary_presence_lookup_id: Option<String>,
}

impl HistoricalAdherenceExceptionInfo {
    pub fn new() -> HistoricalAdherenceExceptionInfo {
        HistoricalAdherenceExceptionInfo {
            start_offset_seconds: None,
            end_offset_seconds: None,
            scheduled_activity_code_id: None,
            scheduled_activity_category: None,
            actual_activity_category: None,
            system_presence: None,
            routing_status: None,
            impact: None,
            secondary_presence_lookup_id: None,
        }
    }
}

/// Activity for which the user is scheduled
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScheduledActivityCategory {
    #[serde(rename = "OnQueueWork")]
    OnQueueWork,
    #[serde(rename = "Break")]
    _Break,
    #[serde(rename = "Meal")]
    Meal,
    #[serde(rename = "Meeting")]
    Meeting,
    #[serde(rename = "OffQueueWork")]
    OffQueueWork,
    #[serde(rename = "TimeOff")]
    TimeOff,
    #[serde(rename = "Training")]
    Training,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "Unscheduled")]
    Unscheduled,
}

impl Default for ScheduledActivityCategory {
    fn default() -> ScheduledActivityCategory {
        Self::OnQueueWork
    }
}
/// Activity for which the user is actually engaged
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActualActivityCategory {
    #[serde(rename = "OnQueueWork")]
    OnQueueWork,
    #[serde(rename = "Break")]
    _Break,
    #[serde(rename = "Meal")]
    Meal,
    #[serde(rename = "Meeting")]
    Meeting,
    #[serde(rename = "OffQueueWork")]
    OffQueueWork,
    #[serde(rename = "TimeOff")]
    TimeOff,
    #[serde(rename = "Training")]
    Training,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "Unscheduled")]
    Unscheduled,
}

impl Default for ActualActivityCategory {
    fn default() -> ActualActivityCategory {
        Self::OnQueueWork
    }
}
/// Actual underlying system presence value
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemPresence {
    #[serde(rename = "Available")]
    Available,
    #[serde(rename = "Away")]
    Away,
    #[serde(rename = "Busy")]
    Busy,
    #[serde(rename = "Offline")]
    Offline,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "OnQueue")]
    OnQueue,
    #[serde(rename = "Meal")]
    Meal,
    #[serde(rename = "Training")]
    Training,
    #[serde(rename = "Meeting")]
    Meeting,
    #[serde(rename = "Break")]
    _Break,
}

impl Default for SystemPresence {
    fn default() -> SystemPresence {
        Self::Available
    }
}
/// The impact of the current adherence state for this user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Impact {
    #[serde(rename = "Positive")]
    Positive,
    #[serde(rename = "Negative")]
    Negative,
    #[serde(rename = "Neutral")]
    Neutral,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for Impact {
    fn default() -> Impact {
        Self::Positive
    }
}

