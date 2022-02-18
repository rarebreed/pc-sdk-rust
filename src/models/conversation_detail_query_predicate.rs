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
pub struct ConversationDetailQueryPredicate {
    /// Optional type, can usually be inferred
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Left hand side for dimension predicates
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    /// Left hand side for metric predicates
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<Metric>,
    /// Optional operator, default is matches
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    /// Right hand side for dimension or metric predicates
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<crate::models::NumericRange>>,
}

impl ConversationDetailQueryPredicate {
    pub fn new() -> ConversationDetailQueryPredicate {
        ConversationDetailQueryPredicate {
            _type: None,
            dimension: None,
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
    #[serde(rename = "conversationEnd")]
    ConversationEnd,
    #[serde(rename = "conversationId")]
    ConversationId,
    #[serde(rename = "conversationInitiator")]
    ConversationInitiator,
    #[serde(rename = "conversationStart")]
    ConversationStart,
    #[serde(rename = "customerParticipation")]
    CustomerParticipation,
    #[serde(rename = "divisionId")]
    DivisionId,
    #[serde(rename = "externalTag")]
    ExternalTag,
    #[serde(rename = "mediaStatsMinConversationMos")]
    MediaStatsMinConversationMos,
    #[serde(rename = "originatingDirection")]
    OriginatingDirection,
}

impl Default for Dimension {
    fn default() -> Dimension {
        Self::ConversationEnd
    }
}
/// Left hand side for metric predicates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metric {
    #[serde(rename = "nBlindTransferred")]
    NBlindTransferred,
    #[serde(rename = "nCobrowseSessions")]
    NCobrowseSessions,
    #[serde(rename = "nConnected")]
    NConnected,
    #[serde(rename = "nConsult")]
    NConsult,
    #[serde(rename = "nConsultTransferred")]
    NConsultTransferred,
    #[serde(rename = "nError")]
    NError,
    #[serde(rename = "nFlow")]
    NFlow,
    #[serde(rename = "nFlowMilestone")]
    NFlowMilestone,
    #[serde(rename = "nFlowOutcome")]
    NFlowOutcome,
    #[serde(rename = "nFlowOutcomeFailed")]
    NFlowOutcomeFailed,
    #[serde(rename = "nOffered")]
    NOffered,
    #[serde(rename = "nOutbound")]
    NOutbound,
    #[serde(rename = "nOutboundAbandoned")]
    NOutboundAbandoned,
    #[serde(rename = "nOutboundAttempted")]
    NOutboundAttempted,
    #[serde(rename = "nOutboundConnected")]
    NOutboundConnected,
    #[serde(rename = "nOverSla")]
    NOverSla,
    #[serde(rename = "nStateTransitionError")]
    NStateTransitionError,
    #[serde(rename = "nTransferred")]
    NTransferred,
    #[serde(rename = "oExternalMediaCount")]
    OExternalMediaCount,
    #[serde(rename = "oFlowMilestone")]
    OFlowMilestone,
    #[serde(rename = "oMediaCount")]
    OMediaCount,
    #[serde(rename = "oMessageTurn")]
    OMessageTurn,
    #[serde(rename = "tAbandon")]
    TAbandon,
    #[serde(rename = "tAcd")]
    TAcd,
    #[serde(rename = "tAcw")]
    TAcw,
    #[serde(rename = "tAgentResponseTime")]
    TAgentResponseTime,
    #[serde(rename = "tAlert")]
    TAlert,
    #[serde(rename = "tAnswered")]
    TAnswered,
    #[serde(rename = "tCallback")]
    TCallback,
    #[serde(rename = "tCallbackComplete")]
    TCallbackComplete,
    #[serde(rename = "tCoaching")]
    TCoaching,
    #[serde(rename = "tCoachingComplete")]
    TCoachingComplete,
    #[serde(rename = "tConnected")]
    TConnected,
    #[serde(rename = "tContacting")]
    TContacting,
    #[serde(rename = "tConversationDuration")]
    TConversationDuration,
    #[serde(rename = "tDialing")]
    TDialing,
    #[serde(rename = "tFlow")]
    TFlow,
    #[serde(rename = "tFlowDisconnect")]
    TFlowDisconnect,
    #[serde(rename = "tFlowExit")]
    TFlowExit,
    #[serde(rename = "tFlowOut")]
    TFlowOut,
    #[serde(rename = "tFlowOutcome")]
    TFlowOutcome,
    #[serde(rename = "tHandle")]
    THandle,
    #[serde(rename = "tHeld")]
    THeld,
    #[serde(rename = "tHeldComplete")]
    THeldComplete,
    #[serde(rename = "tIvr")]
    TIvr,
    #[serde(rename = "tMonitoring")]
    TMonitoring,
    #[serde(rename = "tMonitoringComplete")]
    TMonitoringComplete,
    #[serde(rename = "tNotResponding")]
    TNotResponding,
    #[serde(rename = "tShortAbandon")]
    TShortAbandon,
    #[serde(rename = "tTalk")]
    TTalk,
    #[serde(rename = "tTalkComplete")]
    TTalkComplete,
    #[serde(rename = "tUserResponseTime")]
    TUserResponseTime,
    #[serde(rename = "tVoicemail")]
    TVoicemail,
}

impl Default for Metric {
    fn default() -> Metric {
        Self::NBlindTransferred
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

