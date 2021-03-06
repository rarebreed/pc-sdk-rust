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
pub struct PatchPredictorRequest {
    /// Number of seconds allocated to predictive routing before attempting a different routing method. This is a value between 12 and 900 seconds.
    #[serde(rename = "routingTimeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub routing_timeout_seconds: Option<i32>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::PredictorSchedule>>,
    #[serde(rename = "workloadBalancingConfig", skip_serializing_if = "Option::is_none")]
    pub workload_balancing_config: Option<Box<crate::models::PredictorWorkloadBalancing>>,
}

impl PatchPredictorRequest {
    pub fn new() -> PatchPredictorRequest {
        PatchPredictorRequest {
            routing_timeout_seconds: None,
            schedule: None,
            workload_balancing_config: None,
        }
    }
}


