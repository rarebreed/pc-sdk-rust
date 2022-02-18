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
pub struct TrunkMetricsQoS {
    /// Total number of QoS mismatches over the course of the last 24-hour period (sliding window).
    #[serde(rename = "mismatchCount")]
    pub mismatch_count: i32,
}

impl TrunkMetricsQoS {
    pub fn new(mismatch_count: i32) -> TrunkMetricsQoS {
        TrunkMetricsQoS {
            mismatch_count,
        }
    }
}

