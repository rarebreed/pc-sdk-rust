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
pub struct FlowDiagnosticInfo {
    /// The step number of the survey invite flow where the error occurred.
    #[serde(rename = "lastActionId", skip_serializing_if = "Option::is_none")]
    pub last_action_id: Option<i32>,
}

impl FlowDiagnosticInfo {
    pub fn new() -> FlowDiagnosticInfo {
        FlowDiagnosticInfo {
            last_action_id: None,
        }
    }
}


