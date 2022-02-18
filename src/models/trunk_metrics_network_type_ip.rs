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
pub struct TrunkMetricsNetworkTypeIp {
    /// Assigned IP Address for the interface
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "errorInfo", skip_serializing_if = "Option::is_none")]
    pub error_info: Option<Box<crate::models::TrunkErrorInfo>>,
}

impl TrunkMetricsNetworkTypeIp {
    pub fn new() -> TrunkMetricsNetworkTypeIp {
        TrunkMetricsNetworkTypeIp {
            address: None,
            error_info: None,
        }
    }
}

