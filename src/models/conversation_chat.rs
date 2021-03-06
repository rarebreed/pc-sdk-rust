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
pub struct ConversationChat {
    /// The connection state of this communication.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// A globally unique identifier for this communication.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The room id for the chat.
    #[serde(rename = "roomId", skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    /// A globally unique identifier for the recording associated with this chat.
    #[serde(rename = "recordingId", skip_serializing_if = "Option::is_none")]
    pub recording_id: Option<String>,
    /// The time line of the participant's chat, divided into activity segments.
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<crate::models::Segment>>,
    /// True if this call is held and the person on this side hears silence.
    #[serde(rename = "held", skip_serializing_if = "Option::is_none")]
    pub held: Option<bool>,
    /// The direction of the chat
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// System defined string indicating what caused the communication to disconnect. Will be null until the communication disconnects.
    #[serde(rename = "disconnectType", skip_serializing_if = "Option::is_none")]
    pub disconnect_type: Option<DisconnectType>,
    /// The timestamp the chat was placed on hold in the cloud clock if the chat is currently on hold. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startHoldTime", skip_serializing_if = "Option::is_none")]
    pub start_hold_time: Option<String>,
    /// The timestamp the communication has when it is first put into an alerting state. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startAlertingTime", skip_serializing_if = "Option::is_none")]
    pub start_alerting_time: Option<String>,
    /// The timestamp when this communication was connected in the cloud clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "connectedTime", skip_serializing_if = "Option::is_none")]
    pub connected_time: Option<String>,
    /// The timestamp when this communication disconnected from the conversation in the provider clock. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "disconnectedTime", skip_serializing_if = "Option::is_none")]
    pub disconnected_time: Option<String>,
    /// The source provider for the email.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// The UUID of the script to use.
    #[serde(rename = "scriptId", skip_serializing_if = "Option::is_none")]
    pub script_id: Option<String>,
    /// The id of the peer communication corresponding to a matching leg for this communication.
    #[serde(rename = "peerId", skip_serializing_if = "Option::is_none")]
    pub peer_id: Option<String>,
    /// If available, the URI to the avatar image of this communication.
    #[serde(rename = "avatarImageUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_image_url: Option<String>,
    #[serde(rename = "journeyContext", skip_serializing_if = "Option::is_none")]
    pub journey_context: Option<Box<crate::models::JourneyContext>>,
    #[serde(rename = "wrapup", skip_serializing_if = "Option::is_none")]
    pub wrapup: Option<Box<crate::models::Wrapup>>,
    #[serde(rename = "afterCallWork", skip_serializing_if = "Option::is_none")]
    pub after_call_work: Option<Box<crate::models::AfterCallWork>>,
    /// Indicates if after-call work is required for a communication. Only used when the ACW Setting is Agent Requested.
    #[serde(rename = "afterCallWorkRequired", skip_serializing_if = "Option::is_none")]
    pub after_call_work_required: Option<bool>,
}

impl ConversationChat {
    pub fn new() -> ConversationChat {
        ConversationChat {
            state: None,
            id: None,
            room_id: None,
            recording_id: None,
            segments: None,
            held: None,
            direction: None,
            disconnect_type: None,
            start_hold_time: None,
            start_alerting_time: None,
            connected_time: None,
            disconnected_time: None,
            provider: None,
            script_id: None,
            peer_id: None,
            avatar_image_url: None,
            journey_context: None,
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
    #[serde(rename = "none")]
    None,
}

impl Default for State {
    fn default() -> State {
        Self::Alerting
    }
}
/// The direction of the chat
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Inbound
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
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "transfer.conference")]
    TransferConference,
    #[serde(rename = "transfer.consult")]
    TransferConsult,
    #[serde(rename = "transfer.forward")]
    TransferForward,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "peer")]
    Peer,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "uncallable")]
    Uncallable,
    #[serde(rename = "timeout")]
    Timeout,
}

impl Default for DisconnectType {
    fn default() -> DisconnectType {
        Self::Endpoint
    }
}

