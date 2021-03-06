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
pub struct ShiftTradeNotification {
    /// The start date of the schedule with which this trade is associated
    #[serde(rename = "weekDate", skip_serializing_if = "Option::is_none")]
    pub week_date: Option<String>,
    /// The ID of the shift trade
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    /// Whether this is a one sided shift trade
    #[serde(rename = "oneSided", skip_serializing_if = "Option::is_none")]
    pub one_sided: Option<bool>,
    /// The new state of the shift trade, null if there was no change
    #[serde(rename = "newState", skip_serializing_if = "Option::is_none")]
    pub new_state: Option<NewState>,
    #[serde(rename = "initiatingUser", skip_serializing_if = "Option::is_none")]
    pub initiating_user: Option<Box<crate::models::UserReference>>,
    /// The start date and time of the initiating shift. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "initiatingShiftDate", skip_serializing_if = "Option::is_none")]
    pub initiating_shift_date: Option<String>,
    #[serde(rename = "receivingUser", skip_serializing_if = "Option::is_none")]
    pub receiving_user: Option<Box<crate::models::UserReference>>,
    /// The start date and time of the receiving shift (null if not matched or if one-sided. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "receivingShiftDate", skip_serializing_if = "Option::is_none")]
    pub receiving_shift_date: Option<String>,
}

impl ShiftTradeNotification {
    pub fn new() -> ShiftTradeNotification {
        ShiftTradeNotification {
            week_date: None,
            trade_id: None,
            one_sided: None,
            new_state: None,
            initiating_user: None,
            initiating_shift_date: None,
            receiving_user: None,
            receiving_shift_date: None,
        }
    }
}

/// The new state of the shift trade, null if there was no change
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NewState {
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

impl Default for NewState {
    fn default() -> NewState {
        Self::Unmatched
    }
}

