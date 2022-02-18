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
pub struct ConversationAggregationView {
    /// Target metric name
    #[serde(rename = "target")]
    pub target: Target,
    /// A unique name for this view. Must be distinct from other views and built-in metric names.
    #[serde(rename = "name")]
    pub name: String,
    /// Type of view you wish to create
    #[serde(rename = "function")]
    pub function: Function,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<crate::models::AggregationRange>>,
}

impl ConversationAggregationView {
    pub fn new(target: Target, name: String, function: Function) -> ConversationAggregationView {
        ConversationAggregationView {
            target,
            name,
            function,
            range: None,
        }
    }
}

/// Target metric name
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Target {
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
    #[serde(rename = "oMediaCount")]
    OMediaCount,
    #[serde(rename = "oMessageTurn")]
    OMessageTurn,
    #[serde(rename = "oServiceLevel")]
    OServiceLevel,
    #[serde(rename = "oServiceTarget")]
    OServiceTarget,
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
    #[serde(rename = "tDialing")]
    TDialing,
    #[serde(rename = "tFlowOut")]
    TFlowOut,
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
    #[serde(rename = "tWait")]
    TWait,
}

impl Default for Target {
    fn default() -> Target {
        Self::NBlindTransferred
    }
}
/// Type of view you wish to create
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Function {
    #[serde(rename = "rangeBound")]
    RangeBound,
}

impl Default for Function {
    fn default() -> Function {
        Self::RangeBound
    }
}
