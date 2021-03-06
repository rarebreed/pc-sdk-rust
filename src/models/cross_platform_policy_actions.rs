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
pub struct CrossPlatformPolicyActions {
    /// true to retain the recording associated with the conversation. Default = true
    #[serde(rename = "retainRecording", skip_serializing_if = "Option::is_none")]
    pub retain_recording: Option<bool>,
    /// true to delete the recording associated with the conversation. If retainRecording = true, this will be ignored. Default = false
    #[serde(rename = "deleteRecording", skip_serializing_if = "Option::is_none")]
    pub delete_recording: Option<bool>,
    /// true to delete the recording associated with the conversation regardless of the values of retainRecording or deleteRecording. Default = false
    #[serde(rename = "alwaysDelete", skip_serializing_if = "Option::is_none")]
    pub always_delete: Option<bool>,
    #[serde(rename = "assignEvaluations", skip_serializing_if = "Option::is_none")]
    pub assign_evaluations: Option<Vec<crate::models::EvaluationAssignment>>,
    #[serde(rename = "assignMeteredEvaluations", skip_serializing_if = "Option::is_none")]
    pub assign_metered_evaluations: Option<Vec<crate::models::MeteredEvaluationAssignment>>,
    #[serde(rename = "assignMeteredAssignmentByAgent", skip_serializing_if = "Option::is_none")]
    pub assign_metered_assignment_by_agent: Option<Vec<crate::models::MeteredAssignmentByAgent>>,
    #[serde(rename = "assignCalibrations", skip_serializing_if = "Option::is_none")]
    pub assign_calibrations: Option<Vec<crate::models::CalibrationAssignment>>,
    #[serde(rename = "retentionDuration", skip_serializing_if = "Option::is_none")]
    pub retention_duration: Option<Box<crate::models::RetentionDuration>>,
    #[serde(rename = "mediaTranscriptions", skip_serializing_if = "Option::is_none")]
    pub media_transcriptions: Option<Vec<crate::models::MediaTranscription>>,
    #[serde(rename = "integrationExport", skip_serializing_if = "Option::is_none")]
    pub integration_export: Option<Box<crate::models::IntegrationExport>>,
}

impl CrossPlatformPolicyActions {
    pub fn new() -> CrossPlatformPolicyActions {
        CrossPlatformPolicyActions {
            retain_recording: None,
            delete_recording: None,
            always_delete: None,
            assign_evaluations: None,
            assign_metered_evaluations: None,
            assign_metered_assignment_by_agent: None,
            assign_calibrations: None,
            retention_duration: None,
            media_transcriptions: None,
            integration_export: None,
        }
    }
}


