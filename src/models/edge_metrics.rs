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
pub struct EdgeMetrics {
    #[serde(rename = "edge", skip_serializing_if = "Option::is_none")]
    pub edge: Option<Box<crate::models::DomainEntityRef>>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(rename = "upTimeMsec", skip_serializing_if = "Option::is_none")]
    pub up_time_msec: Option<i64>,
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<crate::models::EdgeMetricsProcessor>>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Vec<crate::models::EdgeMetricsMemory>>,
    #[serde(rename = "disks", skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<crate::models::EdgeMetricsDisk>>,
    #[serde(rename = "subsystems", skip_serializing_if = "Option::is_none")]
    pub subsystems: Option<Vec<crate::models::EdgeMetricsSubsystem>>,
    #[serde(rename = "networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::EdgeMetricsNetwork>>,
}

impl EdgeMetrics {
    pub fn new() -> EdgeMetrics {
        EdgeMetrics {
            edge: None,
            event_time: None,
            up_time_msec: None,
            processors: None,
            memory: None,
            disks: None,
            subsystems: None,
            networks: None,
        }
    }
}

