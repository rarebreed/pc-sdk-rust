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
pub struct SegmentDetailQueryPredicate {
    /// Optional type, can usually be inferred
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Left hand side for dimension predicates
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    /// Left hand side for property predicates
    #[serde(rename = "propertyType", skip_serializing_if = "Option::is_none")]
    pub property_type: Option<PropertyType>,
    /// Left hand side for property predicates
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Left hand side for metric predicates
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<Metric>,
    /// Optional operator, default is matches
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    /// Right hand side for dimension, metric, or property predicates
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<crate::models::NumericRange>>,
}

impl SegmentDetailQueryPredicate {
    pub fn new() -> SegmentDetailQueryPredicate {
        SegmentDetailQueryPredicate {
            _type: None,
            dimension: None,
            property_type: None,
            property: None,
            metric: None,
            operator: None,
            value: None,
            range: None,
        }
    }
}

/// Optional type, can usually be inferred
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "dimension")]
    Dimension,
    #[serde(rename = "property")]
    Property,
    #[serde(rename = "metric")]
    Metric,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dimension
    }
}
/// Left hand side for dimension predicates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Dimension {
    #[serde(rename = "addressFrom")]
    AddressFrom,
    #[serde(rename = "addressTo")]
    AddressTo,
    #[serde(rename = "agentAssistantId")]
    AgentAssistantId,
    #[serde(rename = "agentOwned")]
    AgentOwned,
    #[serde(rename = "ani")]
    Ani,
    #[serde(rename = "authenticated")]
    Authenticated,
    #[serde(rename = "callbackNumber")]
    CallbackNumber,
    #[serde(rename = "callbackScheduledTime")]
    CallbackScheduledTime,
    #[serde(rename = "coachedParticipantId")]
    CoachedParticipantId,
    #[serde(rename = "conference")]
    Conference,
    #[serde(rename = "deliveryStatus")]
    DeliveryStatus,
    #[serde(rename = "destinationAddress")]
    DestinationAddress,
    #[serde(rename = "destinationConversationId")]
    DestinationConversationId,
    #[serde(rename = "direction")]
    Direction,
    #[serde(rename = "disconnectType")]
    DisconnectType,
    #[serde(rename = "dnis")]
    Dnis,
    #[serde(rename = "edgeId")]
    EdgeId,
    #[serde(rename = "errorCode")]
    ErrorCode,
    #[serde(rename = "exitReason")]
    ExitReason,
    #[serde(rename = "extendedDeliveryStatus")]
    ExtendedDeliveryStatus,
    #[serde(rename = "externalContactId")]
    ExternalContactId,
    #[serde(rename = "externalOrganizationId")]
    ExternalOrganizationId,
    #[serde(rename = "flaggedReason")]
    FlaggedReason,
    #[serde(rename = "flowId")]
    FlowId,
    #[serde(rename = "flowName")]
    FlowName,
    #[serde(rename = "flowOutType")]
    FlowOutType,
    #[serde(rename = "flowOutcome")]
    FlowOutcome,
    #[serde(rename = "flowOutcomeId")]
    FlowOutcomeId,
    #[serde(rename = "flowOutcomeValue")]
    FlowOutcomeValue,
    #[serde(rename = "flowVersion")]
    FlowVersion,
    #[serde(rename = "groupId")]
    GroupId,
    #[serde(rename = "journeyActionId")]
    JourneyActionId,
    #[serde(rename = "journeyActionMapId")]
    JourneyActionMapId,
    #[serde(rename = "journeyCustomerId")]
    JourneyCustomerId,
    #[serde(rename = "journeyCustomerIdType")]
    JourneyCustomerIdType,
    #[serde(rename = "journeyCustomerSessionId")]
    JourneyCustomerSessionId,
    #[serde(rename = "mediaCount")]
    MediaCount,
    #[serde(rename = "mediaType")]
    MediaType,
    #[serde(rename = "messageType")]
    MessageType,
    #[serde(rename = "monitoredParticipantId")]
    MonitoredParticipantId,
    #[serde(rename = "outboundCampaignId")]
    OutboundCampaignId,
    #[serde(rename = "outboundContactId")]
    OutboundContactId,
    #[serde(rename = "outboundContactListId")]
    OutboundContactListId,
    #[serde(rename = "participantName")]
    ParticipantName,
    #[serde(rename = "protocolCallId")]
    ProtocolCallId,
    #[serde(rename = "provider")]
    Provider,
    #[serde(rename = "purpose")]
    Purpose,
    #[serde(rename = "queueId")]
    QueueId,
    #[serde(rename = "recording")]
    Recording,
    #[serde(rename = "remote")]
    Remote,
    #[serde(rename = "remoteNameDisplayable")]
    RemoteNameDisplayable,
    #[serde(rename = "requestedLanguageId")]
    RequestedLanguageId,
    #[serde(rename = "requestedRouting")]
    RequestedRouting,
    #[serde(rename = "requestedRoutingSkillId")]
    RequestedRoutingSkillId,
    #[serde(rename = "scoredAgentId")]
    ScoredAgentId,
    #[serde(rename = "scriptId")]
    ScriptId,
    #[serde(rename = "segmentEnd")]
    SegmentEnd,
    #[serde(rename = "segmentType")]
    SegmentType,
    #[serde(rename = "sessionDnis")]
    SessionDnis,
    #[serde(rename = "sipResponseCode")]
    SipResponseCode,
    #[serde(rename = "subject")]
    Subject,
    #[serde(rename = "teamId")]
    TeamId,
    #[serde(rename = "transferTargetAddress")]
    TransferTargetAddress,
    #[serde(rename = "transferTargetName")]
    TransferTargetName,
    #[serde(rename = "transferType")]
    TransferType,
    #[serde(rename = "usedRouting")]
    UsedRouting,
    #[serde(rename = "userId")]
    UserId,
    #[serde(rename = "wrapUpCode")]
    WrapUpCode,
    #[serde(rename = "wrapUpNote")]
    WrapUpNote,
}

impl Default for Dimension {
    fn default() -> Dimension {
        Self::AddressFrom
    }
}
/// Left hand side for property predicates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "real")]
    Real,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "uuid")]
    Uuid,
}

impl Default for PropertyType {
    fn default() -> PropertyType {
        Self::Bool
    }
}
/// Left hand side for metric predicates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metric {
    #[serde(rename = "tSegmentDuration")]
    TSegmentDuration,
}

impl Default for Metric {
    fn default() -> Metric {
        Self::TSegmentDuration
    }
}
/// Optional operator, default is matches
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "matches")]
    Matches,
    #[serde(rename = "exists")]
    Exists,
    #[serde(rename = "notExists")]
    NotExists,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Matches
    }
}

