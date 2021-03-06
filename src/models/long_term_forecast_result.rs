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
pub struct LongTermForecastResult {
    /// The forecast data broken up by planning group
    #[serde(rename = "planningGroups", skip_serializing_if = "Option::is_none")]
    pub planning_groups: Option<Vec<crate::models::LongTermForecastPlanningGroupData>>,
    /// The reference start date relative to the business unit time zone in this forecast. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "referenceStartDate", skip_serializing_if = "Option::is_none")]
    pub reference_start_date: Option<String>,
    /// The number of weeks in this forecast
    #[serde(rename = "weekCount", skip_serializing_if = "Option::is_none")]
    pub week_count: Option<i32>,
}

impl LongTermForecastResult {
    pub fn new() -> LongTermForecastResult {
        LongTermForecastResult {
            planning_groups: None,
            reference_start_date: None,
            week_count: None,
        }
    }
}


