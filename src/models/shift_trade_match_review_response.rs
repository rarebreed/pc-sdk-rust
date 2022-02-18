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
pub struct ShiftTradeMatchReviewResponse {
    #[serde(rename = "initiatingUser", skip_serializing_if = "Option::is_none")]
    pub initiating_user: Option<Box<crate::models::ShiftTradeMatchReviewUserResponse>>,
    #[serde(rename = "receivingUser", skip_serializing_if = "Option::is_none")]
    pub receiving_user: Option<Box<crate::models::ShiftTradeMatchReviewUserResponse>>,
    /// Constraint violations introduced after being matched that would normally disallow a trade, but which can still be overridden by the shift trade administrator
    #[serde(rename = "violations", skip_serializing_if = "Option::is_none")]
    pub violations: Option<Vec<crate::models::ShiftTradeMatchViolation>>,
    /// Constraint violations associated with this shift trade which require shift trade administrator review
    #[serde(rename = "adminReviewViolations", skip_serializing_if = "Option::is_none")]
    pub admin_review_violations: Option<Vec<crate::models::ShiftTradeMatchViolation>>,
}

impl ShiftTradeMatchReviewResponse {
    pub fn new() -> ShiftTradeMatchReviewResponse {
        ShiftTradeMatchReviewResponse {
            initiating_user: None,
            receiving_user: None,
            violations: None,
            admin_review_violations: None,
        }
    }
}

