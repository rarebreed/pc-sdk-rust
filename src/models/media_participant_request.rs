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
pub struct MediaParticipantRequest {
    #[serde(rename = "wrapup", skip_serializing_if = "Option::is_none")]
    pub wrapup: Option<Box<crate::models::Wrapup>>,
    /// The state to update to set for this participant's communications.  Possible values are: 'connected' and 'disconnected'.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// True to enable recording of this participant, otherwise false to disable recording.
    #[serde(rename = "recording", skip_serializing_if = "Option::is_none")]
    pub recording: Option<bool>,
    /// True to mute this conversation participant.
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    /// True to confine this conversation participant.  Should only be used for ad-hoc conferences
    #[serde(rename = "confined", skip_serializing_if = "Option::is_none")]
    pub confined: Option<bool>,
    /// True to hold this conversation participant.
    #[serde(rename = "held", skip_serializing_if = "Option::is_none")]
    pub held: Option<bool>,
    /// True to skip wrap-up for this participant.
    #[serde(rename = "wrapupSkipped", skip_serializing_if = "Option::is_none")]
    pub wrapup_skipped: Option<bool>,
}

impl MediaParticipantRequest {
    pub fn new() -> MediaParticipantRequest {
        MediaParticipantRequest {
            wrapup: None,
            state: None,
            recording: None,
            muted: None,
            confined: None,
            held: None,
            wrapup_skipped: None,
        }
    }
}

/// The state to update to set for this participant's communications.  Possible values are: 'connected' and 'disconnected'.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "alerting")]
    Alerting,
    #[serde(rename = "dialing")]
    Dialing,
    #[serde(rename = "contacting")]
    Contacting,
    #[serde(rename = "offering")]
    Offering,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "converting")]
    Converting,
    #[serde(rename = "uploading")]
    Uploading,
    #[serde(rename = "transmitting")]
    Transmitting,
    #[serde(rename = "none")]
    None,
}

impl Default for State {
    fn default() -> State {
        Self::Alerting
    }
}

