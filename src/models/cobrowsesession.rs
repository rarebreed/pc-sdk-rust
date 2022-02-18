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
pub struct Cobrowsesession {
    /// The connection state of this communication.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// A globally unique identifier for this communication.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects.
    #[serde(rename = "disconnectType", skip_serializing_if = "Option::is_none")]
    pub disconnect_type: Option<DisconnectType>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<Box<crate::models::Address>>,
    /// The co-browse session ID.
    #[serde(rename = "cobrowseSessionId", skip_serializing_if = "Option::is_none")]
    pub cobrowse_session_id: Option<String>,
    /// This value identifies the role of the co-browse client within the co-browse session (a client is a sharer or a viewer).
    #[serde(rename = "cobrowseRole", skip_serializing_if = "Option::is_none")]
    pub cobrowse_role: Option<String>,
    /// ID of co-browse participants for which this client has been granted control (list is empty if this client cannot control any shared pages).
    #[serde(rename = "controlling", skip_serializing_if = "Option::is_none")]
    pub controlling: Option<Vec<String>>,
    /// The URL that can be used to open co-browse session in web browser.
    #[serde(rename = "viewerUrl", skip_serializing_if = "Option::is_none")]
    pub viewer_url: Option<String>,
    /// The time when the provider event which triggered this conversation update happened in the corrected provider clock (milliseconds since 1970-01-01 00:00:00 UTC). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "providerEventTime", skip_serializing_if = "Option::is_none")]
    pub provider_event_time: Option<String>,
    /// The timestamp the communication has when it is first put into an alerting state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startAlertingTime", skip_serializing_if = "Option::is_none")]
    pub start_alerting_time: Option<String>,
    /// The timestamp when this communication was connected in the cloud clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "connectedTime", skip_serializing_if = "Option::is_none")]
    pub connected_time: Option<String>,
    /// The timestamp when this communication disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "disconnectedTime", skip_serializing_if = "Option::is_none")]
    pub disconnected_time: Option<String>,
    /// The source provider for the co-browse session.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// The id of the peer communication corresponding to a matching leg for this communication.
    #[serde(rename = "peerId", skip_serializing_if = "Option::is_none")]
    pub peer_id: Option<String>,
    /// The time line of the participant's call, divided into activity segments.
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<crate::models::Segment>>,
    #[serde(rename = "wrapup", skip_serializing_if = "Option::is_none")]
    pub wrapup: Option<Box<crate::models::Wrapup>>,
    #[serde(rename = "afterCallWork", skip_serializing_if = "Option::is_none")]
    pub after_call_work: Option<Box<crate::models::AfterCallWork>>,
    /// Indicates if after-call work is required for a communication. Only used when the ACW Setting is Agent Requested.
    #[serde(rename = "afterCallWorkRequired", skip_serializing_if = "Option::is_none")]
    pub after_call_work_required: Option<bool>,
}

impl Cobrowsesession {
    pub fn new() -> Cobrowsesession {
        Cobrowsesession {
            state: None,
            id: None,
            disconnect_type: None,
            _self: None,
            cobrowse_session_id: None,
            cobrowse_role: None,
            controlling: None,
            viewer_url: None,
            provider_event_time: None,
            start_alerting_time: None,
            connected_time: None,
            disconnected_time: None,
            provider: None,
            peer_id: None,
            segments: None,
            wrapup: None,
            after_call_work: None,
            after_call_work_required: None,
        }
    }
}

/// The connection state of this communication.
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
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "none")]
    None,
}

impl Default for State {
    fn default() -> State {
        Self::Alerting
    }
}
/// System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisconnectType {
    #[serde(rename = "endpoint")]
    Endpoint,
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "transfer.conference")]
    TransferConference,
    #[serde(rename = "transfer.consult")]
    TransferConsult,
    #[serde(rename = "transfer.forward")]
    TransferForward,
    #[serde(rename = "transport.failure")]
    TransportFailure,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "peer")]
    Peer,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "uncallable")]
    Uncallable,
}

impl Default for DisconnectType {
    fn default() -> DisconnectType {
        Self::Endpoint
    }
}

