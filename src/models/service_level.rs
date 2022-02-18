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
pub struct ServiceLevel {
    /// The desired Service Level. A value between 0 and 1.
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    /// Service Level target in milliseconds.
    #[serde(rename = "durationMs", skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
}

impl ServiceLevel {
    pub fn new() -> ServiceLevel {
        ServiceLevel {
            percentage: None,
            duration_ms: None,
        }
    }
}


