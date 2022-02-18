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
pub struct ShiftTradeMatchReviewUserResponse {
    /// The minimum weekly paid minutes for this user per the work plan tied to the agent schedule
    #[serde(rename = "weeklyMinimumPaidMinutes", skip_serializing_if = "Option::is_none")]
    pub weekly_minimum_paid_minutes: Option<i32>,
    /// The maximum weekly paid minutes for this user per the work plan tied to the agent schedule
    #[serde(rename = "weeklyMaximumPaidMinutes", skip_serializing_if = "Option::is_none")]
    pub weekly_maximum_paid_minutes: Option<i32>,
    /// The paid minutes on the week schedule for this user prior to the shift trade
    #[serde(rename = "preTradeSchedulePaidMinutes", skip_serializing_if = "Option::is_none")]
    pub pre_trade_schedule_paid_minutes: Option<i32>,
    /// The paid minutes on the week schedule for this user if the shift trade is approved
    #[serde(rename = "postTradeSchedulePaidMinutes", skip_serializing_if = "Option::is_none")]
    pub post_trade_schedule_paid_minutes: Option<i32>,
    #[serde(rename = "postTradeNewShift", skip_serializing_if = "Option::is_none")]
    pub post_trade_new_shift: Option<Box<crate::models::ShiftTradePreviewResponse>>,
}

impl ShiftTradeMatchReviewUserResponse {
    pub fn new() -> ShiftTradeMatchReviewUserResponse {
        ShiftTradeMatchReviewUserResponse {
            weekly_minimum_paid_minutes: None,
            weekly_maximum_paid_minutes: None,
            pre_trade_schedule_paid_minutes: None,
            post_trade_schedule_paid_minutes: None,
            post_trade_new_shift: None,
        }
    }
}

