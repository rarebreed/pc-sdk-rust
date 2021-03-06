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
pub struct TranscriptionSettings {
    /// Setting to enable/disable transcription capability
    #[serde(rename = "transcription")]
    pub transcription: Transcription,
    /// Configure confidence threshold. The possible values are from 1 to 100.
    #[serde(rename = "transcriptionConfidenceThreshold")]
    pub transcription_confidence_threshold: i32,
    /// Setting to enable/disable content search
    #[serde(rename = "contentSearchEnabled", skip_serializing_if = "Option::is_none")]
    pub content_search_enabled: Option<bool>,
}

impl TranscriptionSettings {
    pub fn new(transcription: Transcription, transcription_confidence_threshold: i32) -> TranscriptionSettings {
        TranscriptionSettings {
            transcription,
            transcription_confidence_threshold,
            content_search_enabled: None,
        }
    }
}

/// Setting to enable/disable transcription capability
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Transcription {
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "EnabledGlobally")]
    EnabledGlobally,
    #[serde(rename = "EnabledQueueFlow")]
    EnabledQueueFlow,
}

impl Default for Transcription {
    fn default() -> Transcription {
        Self::Disabled
    }
}

