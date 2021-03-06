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
pub struct FlowAggregationQuery {
    /// Behaves like one clause in a SQL WHERE. Specifies the date and time range of data being queried. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss
    #[serde(rename = "interval")]
    pub interval: String,
    /// Granularity aggregates metrics into subpartitions within the time interval specified. The default granularity is the same duration as the interval. Periods are represented as an ISO-8601 string. For example: P1D or P1DT12H
    #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// Time zone context used to calculate response intervals (this allows resolving DST changes). The interval offset is used even when timeZone is specified. Default is UTC. Time zones are represented as a string of the zone name as found in the IANA time zone database. For example: UTC, Etc/UTC, or Europe/London
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Behaves like a SQL GROUPBY. Allows for multiple levels of grouping as a list of dimensions. Partitions resulting aggregate computations into distinct named subgroups rather than across the entire result set as if it were one group.
    #[serde(rename = "groupBy", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupBy>>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::FlowAggregateQueryFilter>>,
    /// Behaves like a SQL SELECT clause. Only named metrics will be retrieved.
    #[serde(rename = "metrics")]
    pub metrics: Vec<Metrics>,
    /// Flattens any multivalued dimensions used in response groups (e.g. ['a','b','c']->'a,b,c')
    #[serde(rename = "flattenMultivaluedDimensions", skip_serializing_if = "Option::is_none")]
    pub flatten_multivalued_dimensions: Option<bool>,
    /// Custom derived metric views
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<Vec<crate::models::FlowAggregationView>>,
    /// Dimension to use as the alternative timestamp for data in the aggregate.  Choosing \"eventTime\" uses the actual time of the data event.
    #[serde(rename = "alternateTimeDimension", skip_serializing_if = "Option::is_none")]
    pub alternate_time_dimension: Option<AlternateTimeDimension>,
}

impl FlowAggregationQuery {
    pub fn new(interval: String, metrics: Vec<Metrics>) -> FlowAggregationQuery {
        FlowAggregationQuery {
            interval,
            granularity: None,
            time_zone: None,
            group_by: None,
            filter: None,
            metrics,
            flatten_multivalued_dimensions: None,
            views: None,
            alternate_time_dimension: None,
        }
    }
}

/// Behaves like a SQL GROUPBY. Allows for multiple levels of grouping as a list of dimensions. Partitions resulting aggregate computations into distinct named subgroups rather than across the entire result set as if it were one group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupBy {
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

impl Default for GroupBy {
    fn default() -> GroupBy {
        Self::ActiveSkillId
    }
}
/// Behaves like a SQL SELECT clause. Only named metrics will be retrieved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metrics {
    #[serde(rename = "nFlow")]
    NFlow,
    #[serde(rename = "nFlowMilestone")]
    NFlowMilestone,
    #[serde(rename = "nFlowOutcome")]
    NFlowOutcome,
    #[serde(rename = "nFlowOutcomeFailed")]
    NFlowOutcomeFailed,
    #[serde(rename = "oFlowMilestone")]
    OFlowMilestone,
    #[serde(rename = "tFlow")]
    TFlow,
    #[serde(rename = "tFlowDisconnect")]
    TFlowDisconnect,
    #[serde(rename = "tFlowExit")]
    TFlowExit,
    #[serde(rename = "tFlowOutcome")]
    TFlowOutcome,
}

impl Default for Metrics {
    fn default() -> Metrics {
        Self::NFlow
    }
}
/// Dimension to use as the alternative timestamp for data in the aggregate.  Choosing \"eventTime\" uses the actual time of the data event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlternateTimeDimension {
    #[serde(rename = "eventTime")]
    EventTime,
}

impl Default for AlternateTimeDimension {
    fn default() -> AlternateTimeDimension {
        Self::EventTime
    }
}

