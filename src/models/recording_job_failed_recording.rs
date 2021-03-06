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
pub struct RecordingJobFailedRecording {
    #[serde(rename = "conversation", skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Box<crate::models::AddressableEntityRef>>,
    #[serde(rename = "recording", skip_serializing_if = "Option::is_none")]
    pub recording: Option<Box<crate::models::AddressableEntityRef>>,
}

impl RecordingJobFailedRecording {
    pub fn new() -> RecordingJobFailedRecording {
        RecordingJobFailedRecording {
            conversation: None,
            recording: None,
        }
    }
}


