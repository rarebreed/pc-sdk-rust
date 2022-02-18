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
pub struct EdgeMetricsProcessor {
    /// Percent time processor was active.
    #[serde(rename = "activeTimePct", skip_serializing_if = "Option::is_none")]
    pub active_time_pct: Option<f64>,
    /// Machine CPU identifier. 'total' will always be included in the array and is the total of all CPU resources.
    #[serde(rename = "cpuId", skip_serializing_if = "Option::is_none")]
    pub cpu_id: Option<String>,
    /// Percent time processor was idle.
    #[serde(rename = "idleTimePct", skip_serializing_if = "Option::is_none")]
    pub idle_time_pct: Option<f64>,
    /// Percent time processor spent in privileged mode.
    #[serde(rename = "privilegedTimePct", skip_serializing_if = "Option::is_none")]
    pub privileged_time_pct: Option<f64>,
    /// Percent time processor spent in user mode.
    #[serde(rename = "userTimePct", skip_serializing_if = "Option::is_none")]
    pub user_time_pct: Option<f64>,
}

impl EdgeMetricsProcessor {
    pub fn new() -> EdgeMetricsProcessor {
        EdgeMetricsProcessor {
            active_time_pct: None,
            cpu_id: None,
            idle_time_pct: None,
            privileged_time_pct: None,
            user_time_pct: None,
        }
    }
}


