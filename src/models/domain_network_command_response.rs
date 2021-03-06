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
pub struct DomainNetworkCommandResponse {
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(rename = "commandName", skip_serializing_if = "Option::is_none")]
    pub command_name: Option<String>,
    #[serde(rename = "acknowledged", skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<bool>,
    #[serde(rename = "errorInfo", skip_serializing_if = "Option::is_none")]
    pub error_info: Option<Box<crate::models::ErrorDetails>>,
}

impl DomainNetworkCommandResponse {
    pub fn new() -> DomainNetworkCommandResponse {
        DomainNetworkCommandResponse {
            correlation_id: None,
            command_name: None,
            acknowledged: None,
            error_info: None,
        }
    }
}


