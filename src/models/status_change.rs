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
pub struct StatusChange {
    /// The date of this status change. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStatusChanged", skip_serializing_if = "Option::is_none")]
    pub date_status_changed: Option<String>,
    /// The status the change request transitioned to
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The status the change request transitioned from
    #[serde(rename = "previousStatus", skip_serializing_if = "Option::is_none")]
    pub previous_status: Option<PreviousStatus>,
    /// A short message describing the status change
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// If applicable, the user who updated the change request to this status
    #[serde(rename = "changedBy", skip_serializing_if = "Option::is_none")]
    pub changed_by: Option<String>,
    /// The reason for rejecting the limit override request
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<RejectReason>,
}

impl StatusChange {
    pub fn new() -> StatusChange {
        StatusChange {
            date_status_changed: None,
            status: None,
            previous_status: None,
            message: None,
            changed_by: None,
            reject_reason: None,
        }
    }
}

/// The status the change request transitioned to
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Open")]
    Open,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "ImplementingChange")]
    ImplementingChange,
    #[serde(rename = "ChangeImplemented")]
    ChangeImplemented,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Rollback")]
    Rollback,
    #[serde(rename = "ImplementingRollback")]
    ImplementingRollback,
    #[serde(rename = "RollbackImplemented")]
    RollbackImplemented,
}

impl Default for Status {
    fn default() -> Status {
        Self::Open
    }
}
/// The status the change request transitioned from
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PreviousStatus {
    #[serde(rename = "Open")]
    Open,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "ImplementingChange")]
    ImplementingChange,
    #[serde(rename = "ChangeImplemented")]
    ChangeImplemented,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Rollback")]
    Rollback,
    #[serde(rename = "ImplementingRollback")]
    ImplementingRollback,
    #[serde(rename = "RollbackImplemented")]
    RollbackImplemented,
}

impl Default for PreviousStatus {
    fn default() -> PreviousStatus {
        Self::Open
    }
}
/// The reason for rejecting the limit override request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RejectReason {
    #[serde(rename = "AlternativeExists")]
    AlternativeExists,
    #[serde(rename = "IncreaseNotRequired")]
    IncreaseNotRequired,
    #[serde(rename = "PlatformMisuse")]
    PlatformMisuse,
    #[serde(rename = "PlatformStability")]
    PlatformStability,
    #[serde(rename = "OtherReason")]
    OtherReason,
}

impl Default for RejectReason {
    fn default() -> RejectReason {
        Self::AlternativeExists
    }
}

