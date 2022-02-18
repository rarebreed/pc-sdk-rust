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
pub struct BotAggregationView {
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

impl BotAggregationView {
    pub fn new(target: Target, name: String, function: Function) -> BotAggregationView {
        BotAggregationView {
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
    #[serde(rename = "nBotSessions")]
    NBotSessions,
    #[serde(rename = "oBotIntent")]
    OBotIntent,
    #[serde(rename = "oBotSessionQuery")]
    OBotSessionQuery,
    #[serde(rename = "oBotSessionTurn")]
    OBotSessionTurn,
    #[serde(rename = "oBotSlot")]
    OBotSlot,
    #[serde(rename = "tBotDisconnect")]
    TBotDisconnect,
    #[serde(rename = "tBotExit")]
    TBotExit,
    #[serde(rename = "tBotRecognitionFailure")]
    TBotRecognitionFailure,
    #[serde(rename = "tBotSession")]
    TBotSession,
}

impl Default for Target {
    fn default() -> Target {
        Self::NBotSessions
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
