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
pub struct CallbackMediaParticipant {
    /// The unique participant ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The display friendly name of the participant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The participant address.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The time when this participant first joined the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The time when this participant went connected for this media (eg: video connected time). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "connectedTime", skip_serializing_if = "Option::is_none")]
    pub connected_time: Option<String>,
    /// The time when this participant went disconnected for this media (eg: video disconnected time). Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The time when this participant's hold started. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startHoldTime", skip_serializing_if = "Option::is_none")]
    pub start_hold_time: Option<String>,
    /// The participant's purpose.  Values can be: 'agent', 'user', 'customer', 'external', 'acd', 'ivr
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// The participant's state.  Values can be: 'alerting', 'connected', 'disconnected', 'dialing', 'contacting
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The participant's direction.  Values can be: 'inbound' or 'outbound'
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// The reason the participant was disconnected from the conversation.
    #[serde(rename = "disconnectType", skip_serializing_if = "Option::is_none")]
    pub disconnect_type: Option<DisconnectType>,
    /// Value is true when the participant is on hold.
    #[serde(rename = "held", skip_serializing_if = "Option::is_none")]
    pub held: Option<bool>,
    /// Value is true when the participant requires wrap-up.
    #[serde(rename = "wrapupRequired", skip_serializing_if = "Option::is_none")]
    pub wrapup_required: Option<bool>,
    /// The wrap-up prompt indicating the type of wrap-up to be performed.
    #[serde(rename = "wrapupPrompt", skip_serializing_if = "Option::is_none")]
    pub wrapup_prompt: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "queue", skip_serializing_if = "Option::is_none")]
    pub queue: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<Box<crate::models::DomainEntityRef>>,
    /// A list of ad-hoc attributes for the participant.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "errorInfo", skip_serializing_if = "Option::is_none")]
    pub error_info: Option<Box<crate::models::ErrorInfo>>,
    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<Box<crate::models::DomainEntityRef>>,
    /// The amount of time the participant has to complete wrap-up.
    #[serde(rename = "wrapupTimeoutMs", skip_serializing_if = "Option::is_none")]
    pub wrapup_timeout_ms: Option<i32>,
    /// Value is true when the participant has skipped wrap-up.
    #[serde(rename = "wrapupSkipped", skip_serializing_if = "Option::is_none")]
    pub wrapup_skipped: Option<bool>,
    /// Specifies how long the agent has to answer an interaction before being marked as not responding.
    #[serde(rename = "alertingTimeoutMs", skip_serializing_if = "Option::is_none")]
    pub alerting_timeout_ms: Option<i32>,
    /// The source provider for the communication.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "externalContact", skip_serializing_if = "Option::is_none")]
    pub external_contact: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "externalOrganization", skip_serializing_if = "Option::is_none")]
    pub external_organization: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "wrapup", skip_serializing_if = "Option::is_none")]
    pub wrapup: Option<Box<crate::models::Wrapup>>,
    /// The peer communication corresponding to a matching leg for this communication.
    #[serde(rename = "peer", skip_serializing_if = "Option::is_none")]
    pub peer: Option<String>,
    /// The reason specifying why participant flagged the conversation.
    #[serde(rename = "flaggedReason", skip_serializing_if = "Option::is_none")]
    pub flagged_reason: Option<FlaggedReason>,
    #[serde(rename = "journeyContext", skip_serializing_if = "Option::is_none")]
    pub journey_context: Option<Box<crate::models::JourneyContext>>,
    #[serde(rename = "conversationRoutingData", skip_serializing_if = "Option::is_none")]
    pub conversation_routing_data: Option<Box<crate::models::ConversationRoutingData>>,
    /// The timestamp when this participant started after-call work. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startAcwTime", skip_serializing_if = "Option::is_none")]
    pub start_acw_time: Option<String>,
    /// The timestamp when this participant ended after-call work. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "endAcwTime", skip_serializing_if = "Option::is_none")]
    pub end_acw_time: Option<String>,
    #[serde(rename = "outboundPreview", skip_serializing_if = "Option::is_none")]
    pub outbound_preview: Option<Box<crate::models::DialerPreview>>,
    #[serde(rename = "voicemail", skip_serializing_if = "Option::is_none")]
    pub voicemail: Option<Box<crate::models::Voicemail>>,
    /// The list of phone number to use for this callback.
    #[serde(rename = "callbackNumbers", skip_serializing_if = "Option::is_none")]
    pub callback_numbers: Option<Vec<String>>,
    /// The name of the callback target.
    #[serde(rename = "callbackUserName", skip_serializing_if = "Option::is_none")]
    pub callback_user_name: Option<String>,
    /// True if the call for the callback uses external dialing.
    #[serde(rename = "externalCampaign", skip_serializing_if = "Option::is_none")]
    pub external_campaign: Option<bool>,
    /// If true, the callback can be skipped.
    #[serde(rename = "skipEnabled", skip_serializing_if = "Option::is_none")]
    pub skip_enabled: Option<bool>,
    /// Duration in seconds before the callback will be auto-dialed.
    #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// The id of the config for automatically placing the callback (and handling the disposition). If absent, the callback will not be placed automatically but routed to an agent as per normal.
    #[serde(rename = "automatedCallbackConfigId", skip_serializing_if = "Option::is_none")]
    pub automated_callback_config_id: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "callbackScheduledTime", skip_serializing_if = "Option::is_none")]
    pub callback_scheduled_time: Option<String>,
}

impl CallbackMediaParticipant {
    pub fn new() -> CallbackMediaParticipant {
        CallbackMediaParticipant {
            id: None,
            name: None,
            address: None,
            start_time: None,
            connected_time: None,
            end_time: None,
            start_hold_time: None,
            purpose: None,
            state: None,
            direction: None,
            disconnect_type: None,
            held: None,
            wrapup_required: None,
            wrapup_prompt: None,
            user: None,
            queue: None,
            team: None,
            attributes: None,
            error_info: None,
            script: None,
            wrapup_timeout_ms: None,
            wrapup_skipped: None,
            alerting_timeout_ms: None,
            provider: None,
            external_contact: None,
            external_organization: None,
            wrapup: None,
            peer: None,
            flagged_reason: None,
            journey_context: None,
            conversation_routing_data: None,
            start_acw_time: None,
            end_acw_time: None,
            outbound_preview: None,
            voicemail: None,
            callback_numbers: None,
            callback_user_name: None,
            external_campaign: None,
            skip_enabled: None,
            timeout_seconds: None,
            automated_callback_config_id: None,
            callback_scheduled_time: None,
        }
    }
}

/// The participant's state.  Values can be: 'alerting', 'connected', 'disconnected', 'dialing', 'contacting
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
/// The participant's direction.  Values can be: 'inbound' or 'outbound'
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
/// The reason the participant was disconnected from the conversation.
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
    #[serde(rename = "transfer.noanswer")]
    TransferNoanswer,
    #[serde(rename = "transfer.notavailable")]
    TransferNotavailable,
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
}

impl Default for DisconnectType {
    fn default() -> DisconnectType {
        Self::Endpoint
    }
}
/// The reason specifying why participant flagged the conversation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlaggedReason {
    #[serde(rename = "general")]
    General,
}

impl Default for FlaggedReason {
    fn default() -> FlaggedReason {
        Self::General
    }
}

