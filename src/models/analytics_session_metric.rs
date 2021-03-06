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
pub struct AnalyticsSessionMetric {
    /// Metric emission date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "emitDate", skip_serializing_if = "Option::is_none")]
    pub emit_date: Option<String>,
    /// Unique name of this metric
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The metric value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

impl AnalyticsSessionMetric {
    pub fn new() -> AnalyticsSessionMetric {
        AnalyticsSessionMetric {
            emit_date: None,
            name: None,
            value: None,
        }
    }
}


