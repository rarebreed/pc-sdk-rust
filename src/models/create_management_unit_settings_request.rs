/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateManagementUnitSettingsRequest : Management Unit Settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateManagementUnitSettingsRequest {
    #[serde(rename = "adherence", skip_serializing_if = "Option::is_none")]
    pub adherence: Option<Box<crate::models::AdherenceSettings>>,
    #[serde(rename = "shortTermForecasting", skip_serializing_if = "Option::is_none")]
    pub short_term_forecasting: Option<Box<crate::models::ShortTermForecastingSettings>>,
    #[serde(rename = "timeOff", skip_serializing_if = "Option::is_none")]
    pub time_off: Option<Box<crate::models::TimeOffRequestSettings>>,
    #[serde(rename = "scheduling", skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Box<crate::models::SchedulingSettingsRequest>>,
    #[serde(rename = "shiftTrading", skip_serializing_if = "Option::is_none")]
    pub shift_trading: Option<Box<crate::models::ShiftTradeSettings>>,
}

impl CreateManagementUnitSettingsRequest {
    /// Management Unit Settings
    pub fn new() -> CreateManagementUnitSettingsRequest {
        CreateManagementUnitSettingsRequest {
            adherence: None,
            short_term_forecasting: None,
            time_off: None,
            scheduling: None,
            shift_trading: None,
        }
    }
}


