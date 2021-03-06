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
pub struct ConsultTransfer {
    /// Determines to whom the initiating participant is speaking. Defaults to DESTINATION
    #[serde(rename = "speakTo", skip_serializing_if = "Option::is_none")]
    pub speak_to: Option<SpeakTo>,
    #[serde(rename = "destination")]
    pub destination: Box<crate::models::Destination>,
}

impl ConsultTransfer {
    pub fn new(destination: crate::models::Destination) -> ConsultTransfer {
        ConsultTransfer {
            speak_to: None,
            destination: Box::new(destination),
        }
    }
}

/// Determines to whom the initiating participant is speaking. Defaults to DESTINATION
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpeakTo {
    #[serde(rename = "DESTINATION")]
    DESTINATION,
    #[serde(rename = "OBJECT")]
    OBJECT,
    #[serde(rename = "BOTH")]
    BOTH,
}

impl Default for SpeakTo {
    fn default() -> SpeakTo {
        Self::DESTINATION
    }
}

