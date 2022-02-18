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
pub struct BulkUpdateShiftTradeStateResultItem {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The state of the shift trade after the update request is processed
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "reviewedBy", skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<Box<crate::models::UserReference>>,
    /// The date the request was reviewed, if applicable. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "reviewedDate", skip_serializing_if = "Option::is_none")]
    pub reviewed_date: Option<String>,
    /// The reason the update failed, if applicable
    #[serde(rename = "failureReason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<FailureReason>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::WfmVersionedEntityMetadata>>,
}

impl BulkUpdateShiftTradeStateResultItem {
    pub fn new() -> BulkUpdateShiftTradeStateResultItem {
        BulkUpdateShiftTradeStateResultItem {
            id: None,
            state: None,
            reviewed_by: None,
            reviewed_date: None,
            failure_reason: None,
            metadata: None,
        }
    }
}

/// The state of the shift trade after the update request is processed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Unmatched")]
    Unmatched,
    #[serde(rename = "Matched")]
    Matched,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Denied")]
    Denied,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "Canceled")]
    Canceled,
}

impl Default for State {
    fn default() -> State {
        Self::Unmatched
    }
}
/// The reason the update failed, if applicable
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FailureReason {
    #[serde(rename = "InitiatingAgentScheduleNotFound")]
    InitiatingAgentScheduleNotFound,
    #[serde(rename = "InitiatingAgentShiftHasExternalActivities")]
    InitiatingAgentShiftHasExternalActivities,
    #[serde(rename = "InitiatingAgentShiftNotFound")]
    InitiatingAgentShiftNotFound,
    #[serde(rename = "ReceivingAgentNotFound")]
    ReceivingAgentNotFound,
    #[serde(rename = "ReceivingAgentScheduleNotFound")]
    ReceivingAgentScheduleNotFound,
    #[serde(rename = "ReceivingAgentShiftHasExternalActivities")]
    ReceivingAgentShiftHasExternalActivities,
    #[serde(rename = "ReceivingAgentShiftNotFound")]
    ReceivingAgentShiftNotFound,
    #[serde(rename = "ScheduleNotPublished")]
    ScheduleNotPublished,
    #[serde(rename = "TransitionNotAllowed")]
    TransitionNotAllowed,
}

impl Default for FailureReason {
    fn default() -> FailureReason {
        Self::InitiatingAgentScheduleNotFound
    }
}
