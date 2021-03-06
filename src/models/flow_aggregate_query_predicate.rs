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
pub struct FlowAggregateQueryPredicate {
    /// Optional type, can usually be inferred
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Left hand side for dimension predicates
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    /// Optional operator, default is matches
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    /// Right hand side for dimension predicates
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<crate::models::NumericRange>>,
}

impl FlowAggregateQueryPredicate {
    pub fn new() -> FlowAggregateQueryPredicate {
        FlowAggregateQueryPredicate {
            _type: None,
            dimension: None,
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
    #[serde(rename = "activeSkillId")]
    ActiveSkillId,
    #[serde(rename = "addressFrom")]
    AddressFrom,
    #[serde(rename = "addressTo")]
    AddressTo,
    #[serde(rename = "agentAssistantId")]
    AgentAssistantId,
    #[serde(rename = "agentBullseyeRing")]
    AgentBullseyeRing,
    #[serde(rename = "agentOwned")]
    AgentOwned,
    #[serde(rename = "agentRank")]
    AgentRank,
    #[serde(rename = "agentScore")]
    AgentScore,
    #[serde(rename = "ani")]
    Ani,
    #[serde(rename = "assignerId")]
    AssignerId,
    #[serde(rename = "authenticated")]
    Authenticated,
    #[serde(rename = "conversationId")]
    ConversationId,
    #[serde(rename = "conversationInitiator")]
    ConversationInitiator,
    #[serde(rename = "convertedFrom")]
    ConvertedFrom,
    #[serde(rename = "convertedTo")]
    ConvertedTo,
    #[serde(rename = "customerParticipation")]
    CustomerParticipation,
    #[serde(rename = "deliveryStatus")]
    DeliveryStatus,
    #[serde(rename = "destinationAddress")]
    DestinationAddress,
    #[serde(rename = "direction")]
    Direction,
    #[serde(rename = "disconnectType")]
    DisconnectType,
    #[serde(rename = "divisionId")]
    DivisionId,
    #[serde(rename = "dnis")]
    Dnis,
    #[serde(rename = "edgeId")]
    EdgeId,
    #[serde(rename = "eligibleAgentCount")]
    EligibleAgentCount,
    #[serde(rename = "endingLanguage")]
    EndingLanguage,
    #[serde(rename = "entryReason")]
    EntryReason,
    #[serde(rename = "entryType")]
    EntryType,
    #[serde(rename = "exitReason")]
    ExitReason,
    #[serde(rename = "extendedDeliveryStatus")]
    ExtendedDeliveryStatus,
    #[serde(rename = "externalContactId")]
    ExternalContactId,
    #[serde(rename = "externalMediaCount")]
    ExternalMediaCount,
    #[serde(rename = "externalOrganizationId")]
    ExternalOrganizationId,
    #[serde(rename = "externalTag")]
    ExternalTag,
    #[serde(rename = "firstQueue")]
    FirstQueue,
    #[serde(rename = "flaggedReason")]
    FlaggedReason,
    #[serde(rename = "flowId")]
    FlowId,
    #[serde(rename = "flowInType")]
    FlowInType,
    #[serde(rename = "flowMilestoneId")]
    FlowMilestoneId,
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
    #[serde(rename = "flowType")]
    FlowType,
    #[serde(rename = "flowVersion")]
    FlowVersion,
    #[serde(rename = "groupId")]
    GroupId,
    #[serde(rename = "interactionType")]
    InteractionType,
    #[serde(rename = "journeyActionId")]
    JourneyActionId,
    #[serde(rename = "journeyActionMapId")]
    JourneyActionMapId,
    #[serde(rename = "journeyActionMapVersion")]
    JourneyActionMapVersion,
    #[serde(rename = "journeyCustomerId")]
    JourneyCustomerId,
    #[serde(rename = "journeyCustomerIdType")]
    JourneyCustomerIdType,
    #[serde(rename = "journeyCustomerSessionId")]
    JourneyCustomerSessionId,
    #[serde(rename = "journeyCustomerSessionIdType")]
    JourneyCustomerSessionIdType,
    #[serde(rename = "knowledgeBaseId")]
    KnowledgeBaseId,
    #[serde(rename = "mediaCount")]
    MediaCount,
    #[serde(rename = "mediaType")]
    MediaType,
    #[serde(rename = "messageType")]
    MessageType,
    #[serde(rename = "originatingDirection")]
    OriginatingDirection,
    #[serde(rename = "outboundCampaignId")]
    OutboundCampaignId,
    #[serde(rename = "outboundContactId")]
    OutboundContactId,
    #[serde(rename = "outboundContactListId")]
    OutboundContactListId,
    #[serde(rename = "participantName")]
    ParticipantName,
    #[serde(rename = "peerId")]
    PeerId,
    #[serde(rename = "proposedAgentId")]
    ProposedAgentId,
    #[serde(rename = "provider")]
    Provider,
    #[serde(rename = "purpose")]
    Purpose,
    #[serde(rename = "queueId")]
    QueueId,
    #[serde(rename = "recognitionFailureReason")]
    RecognitionFailureReason,
    #[serde(rename = "remote")]
    Remote,
    #[serde(rename = "removedSkillId")]
    RemovedSkillId,
    #[serde(rename = "reoffered")]
    Reoffered,
    #[serde(rename = "requestedLanguageId")]
    RequestedLanguageId,
    #[serde(rename = "requestedRouting")]
    RequestedRouting,
    #[serde(rename = "requestedRoutingSkillId")]
    RequestedRoutingSkillId,
    #[serde(rename = "roomId")]
    RoomId,
    #[serde(rename = "routingPriority")]
    RoutingPriority,
    #[serde(rename = "routingRing")]
    RoutingRing,
    #[serde(rename = "scoredAgentId")]
    ScoredAgentId,
    #[serde(rename = "selectedAgentId")]
    SelectedAgentId,
    #[serde(rename = "selectedAgentRank")]
    SelectedAgentRank,
    #[serde(rename = "selfServed")]
    SelfServed,
    #[serde(rename = "sessionDnis")]
    SessionDnis,
    #[serde(rename = "sessionId")]
    SessionId,
    #[serde(rename = "startingLanguage")]
    StartingLanguage,
    #[serde(rename = "stationId")]
    StationId,
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
    #[serde(rename = "waitingInteractionCount")]
    WaitingInteractionCount,
    #[serde(rename = "wrapUpCode")]
    WrapUpCode,
}

impl Default for Dimension {
    fn default() -> Dimension {
        Self::ActiveSkillId
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

