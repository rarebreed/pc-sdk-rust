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
pub struct CreateSecureSession {
    /// requesting participant
    #[serde(rename = "sourceParticipantId", skip_serializing_if = "Option::is_none")]
    pub source_participant_id: Option<String>,
    /// the flow id to execute in the secure session
    #[serde(rename = "flowId")]
    pub flow_id: String,
    /// user data for the secure session
    #[serde(rename = "userData")]
    pub user_data: String,
    /// if true, disconnect the agent after creating the session
    #[serde(rename = "disconnect", skip_serializing_if = "Option::is_none")]
    pub disconnect: Option<bool>,
}

impl CreateSecureSession {
    pub fn new(flow_id: String, user_data: String) -> CreateSecureSession {
        CreateSecureSession {
            source_participant_id: None,
            flow_id,
            user_data,
            disconnect: None,
        }
    }
}


