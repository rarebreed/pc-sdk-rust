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
pub struct UpdateTimeOffPlanRequest {
    /// The name of this time off plan.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "activityCodeIds", skip_serializing_if = "Option::is_none")]
    pub activity_code_ids: Option<Box<crate::models::SetWrapperString>>,
    #[serde(rename = "timeOffLimitIds", skip_serializing_if = "Option::is_none")]
    pub time_off_limit_ids: Option<Box<crate::models::SetWrapperString>>,
    /// Auto approval rule for the time off plan.
    #[serde(rename = "autoApprovalRule", skip_serializing_if = "Option::is_none")]
    pub auto_approval_rule: Option<AutoApprovalRule>,
    /// The number of days before the time off request start date for when the request will be expired from the waitlist.
    #[serde(rename = "daysBeforeStartToExpireFromWaitlist", skip_serializing_if = "Option::is_none")]
    pub days_before_start_to_expire_from_waitlist: Option<i32>,
    /// Whether this time off plan should be used by agents.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::WfmVersionedEntityMetadata>,
}

impl UpdateTimeOffPlanRequest {
    pub fn new(metadata: crate::models::WfmVersionedEntityMetadata) -> UpdateTimeOffPlanRequest {
        UpdateTimeOffPlanRequest {
            name: None,
            activity_code_ids: None,
            time_off_limit_ids: None,
            auto_approval_rule: None,
            days_before_start_to_expire_from_waitlist: None,
            active: None,
            metadata: Box::new(metadata),
        }
    }
}

/// Auto approval rule for the time off plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutoApprovalRule {
    #[serde(rename = "Never")]
    Never,
    #[serde(rename = "Always")]
    Always,
    #[serde(rename = "CheckLimits")]
    CheckLimits,
}

impl Default for AutoApprovalRule {
    fn default() -> AutoApprovalRule {
        Self::Never
    }
}

