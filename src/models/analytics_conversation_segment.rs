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
pub struct AnalyticsConversationSegment {
    /// Flag indicating if audio is muted or not (true/false)
    #[serde(rename = "audioMuted", skip_serializing_if = "Option::is_none")]
    pub audio_muted: Option<bool>,
    /// Indicates whether the segment was a conference
    #[serde(rename = "conference", skip_serializing_if = "Option::is_none")]
    pub conference: Option<bool>,
    /// The unique identifier of a new conversation when a conversation is ended for a conference
    #[serde(rename = "destinationConversationId", skip_serializing_if = "Option::is_none")]
    pub destination_conversation_id: Option<String>,
    /// The unique identifier of a new session when a session is ended for a conference
    #[serde(rename = "destinationSessionId", skip_serializing_if = "Option::is_none")]
    pub destination_session_id: Option<String>,
    /// The session disconnect type
    #[serde(rename = "disconnectType", skip_serializing_if = "Option::is_none")]
    pub disconnect_type: Option<DisconnectType>,
    /// A code corresponding to the error that occurred
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Unique identifier for a PureCloud group
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Q.850 response code(s)
    #[serde(rename = "q850ResponseCodes", skip_serializing_if = "Option::is_none")]
    pub q850_response_codes: Option<Vec<i64>>,
    /// Queue identifier
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    /// Unique identifier for the language requested for an interaction
    #[serde(rename = "requestedLanguageId", skip_serializing_if = "Option::is_none")]
    pub requested_language_id: Option<String>,
    /// Unique identifier(s) for skill(s) requested for an interaction
    #[serde(rename = "requestedRoutingSkillIds", skip_serializing_if = "Option::is_none")]
    pub requested_routing_skill_ids: Option<Vec<String>>,
    /// Unique identifier(s) for agent(s) requested for an interaction
    #[serde(rename = "requestedRoutingUserIds", skip_serializing_if = "Option::is_none")]
    pub requested_routing_user_ids: Option<Vec<String>>,
    /// The end time of a segment. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "segmentEnd", skip_serializing_if = "Option::is_none")]
    pub segment_end: Option<String>,
    /// The start time of a segment. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "segmentStart", skip_serializing_if = "Option::is_none")]
    pub segment_start: Option<String>,
    /// The activity that takes place in the segment, such as hold or interact
    #[serde(rename = "segmentType", skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<SegmentType>,
    /// SIP response code(s)
    #[serde(rename = "sipResponseCodes", skip_serializing_if = "Option::is_none")]
    pub sip_response_codes: Option<Vec<i64>>,
    /// The unique identifier of the previous conversation when a new conversation is created for a conference
    #[serde(rename = "sourceConversationId", skip_serializing_if = "Option::is_none")]
    pub source_conversation_id: Option<String>,
    /// The unique identifier of the previous session when a new session is created for a conference
    #[serde(rename = "sourceSessionId", skip_serializing_if = "Option::is_none")]
    pub source_session_id: Option<String>,
    /// The subject for the initial email that started this conversation
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Flag indicating if video is muted/paused or not (true/false)
    #[serde(rename = "videoMuted", skip_serializing_if = "Option::is_none")]
    pub video_muted: Option<bool>,
    /// Wrap up code
    #[serde(rename = "wrapUpCode", skip_serializing_if = "Option::is_none")]
    pub wrap_up_code: Option<String>,
    /// Note entered by an agent during after-call work
    #[serde(rename = "wrapUpNote", skip_serializing_if = "Option::is_none")]
    pub wrap_up_note: Option<String>,
    /// Tag(s) assigned during after-call work
    #[serde(rename = "wrapUpTags", skip_serializing_if = "Option::is_none")]
    pub wrap_up_tags: Option<Vec<String>>,
    /// Scored agents
    #[serde(rename = "scoredAgents", skip_serializing_if = "Option::is_none")]
    pub scored_agents: Option<Vec<crate::models::AnalyticsScoredAgent>>,
    /// Additional segment properties
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::models::AnalyticsProperty>>,
}

impl AnalyticsConversationSegment {
    pub fn new() -> AnalyticsConversationSegment {
        AnalyticsConversationSegment {
            audio_muted: None,
            conference: None,
            destination_conversation_id: None,
            destination_session_id: None,
            disconnect_type: None,
            error_code: None,
            group_id: None,
            q850_response_codes: None,
            queue_id: None,
            requested_language_id: None,
            requested_routing_skill_ids: None,
            requested_routing_user_ids: None,
            segment_end: None,
            segment_start: None,
            segment_type: None,
            sip_response_codes: None,
            source_conversation_id: None,
            source_session_id: None,
            subject: None,
            video_muted: None,
            wrap_up_code: None,
            wrap_up_note: None,
            wrap_up_tags: None,
            scored_agents: None,
            properties: None,
        }
    }
}

/// The session disconnect type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisconnectType {
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "conferenceTransfer")]
    ConferenceTransfer,
    #[serde(rename = "consultTransfer")]
    ConsultTransfer,
    #[serde(rename = "endpoint")]
    Endpoint,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "forwardTransfer")]
    ForwardTransfer,
    #[serde(rename = "noAnswerTransfer")]
    NoAnswerTransfer,
    #[serde(rename = "notAvailableTransfer")]
    NotAvailableTransfer,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "peer")]
    Peer,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "transportFailure")]
    TransportFailure,
    #[serde(rename = "uncallable")]
    Uncallable,
}

impl Default for DisconnectType {
    fn default() -> DisconnectType {
        Self::Client
    }
}
/// The activity that takes place in the segment, such as hold or interact
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SegmentType {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "callback")]
    Callback,
    #[serde(rename = "coaching")]
    Coaching,
    #[serde(rename = "contacting")]
    Contacting,
    #[serde(rename = "converting")]
    Converting,
    #[serde(rename = "delay")]
    Delay,
    #[serde(rename = "dialing")]
    Dialing,
    #[serde(rename = "hold")]
    Hold,
    #[serde(rename = "interact")]
    Interact,
    #[serde(rename = "ivr")]
    Ivr,
    #[serde(rename = "monitoring")]
    Monitoring,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "sharing")]
    Sharing,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "transmitting")]
    Transmitting,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "uploading")]
    Uploading,
    #[serde(rename = "voicemail")]
    Voicemail,
    #[serde(rename = "wrapup")]
    Wrapup,
}

impl Default for SegmentType {
    fn default() -> SegmentType {
        Self::Alert
    }
}

