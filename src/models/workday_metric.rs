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
pub struct WorkdayMetric {
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<Box<crate::models::Metric>>,
    #[serde(rename = "objective", skip_serializing_if = "Option::is_none")]
    pub objective: Option<Box<crate::models::Objective>>,
    /// Gamification points earned for this metric
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,
    /// Value of this metric
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// List of schedule activity events for punctuality metrics
    #[serde(rename = "punctualityEvents", skip_serializing_if = "Option::is_none")]
    pub punctuality_events: Option<Vec<crate::models::PunctualityEvent>>,
}

impl WorkdayMetric {
    pub fn new() -> WorkdayMetric {
        WorkdayMetric {
            metric: None,
            objective: None,
            points: None,
            value: None,
            punctuality_events: None,
        }
    }
}


