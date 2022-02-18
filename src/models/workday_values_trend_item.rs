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
pub struct WorkdayValuesTrendItem {
    /// The workday for the metric value. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateWorkday", skip_serializing_if = "Option::is_none")]
    pub date_workday: Option<String>,
    /// The metric value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl WorkdayValuesTrendItem {
    pub fn new() -> WorkdayValuesTrendItem {
        WorkdayValuesTrendItem {
            date_workday: None,
            value: None,
        }
    }
}

