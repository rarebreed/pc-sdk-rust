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
pub struct BotAggregationQuery {
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
    pub filter: Option<Box<crate::models::BotAggregateQueryFilter>>,
    /// Behaves like a SQL SELECT clause. Only named metrics will be retrieved.
    #[serde(rename = "metrics")]
    pub metrics: Vec<Metrics>,
    /// Flattens any multivalued dimensions used in response groups (e.g. ['a','b','c']->'a,b,c')
    #[serde(rename = "flattenMultivaluedDimensions", skip_serializing_if = "Option::is_none")]
    pub flatten_multivalued_dimensions: Option<bool>,
    /// Custom derived metric views
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<Vec<crate::models::BotAggregationView>>,
    /// Dimension to use as the alternative timestamp for data in the aggregate.  Choosing \"eventTime\" uses the actual time of the data event.
    #[serde(rename = "alternateTimeDimension", skip_serializing_if = "Option::is_none")]
    pub alternate_time_dimension: Option<AlternateTimeDimension>,
}

impl BotAggregationQuery {
    pub fn new(interval: String, metrics: Vec<Metrics>) -> BotAggregationQuery {
        BotAggregationQuery {
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
    #[serde(rename = "botFinalIntent")]
    BotFinalIntent,
    #[serde(rename = "botId")]
    BotId,
    #[serde(rename = "botIntent")]
    BotIntent,
    #[serde(rename = "botProduct")]
    BotProduct,
    #[serde(rename = "botProvider")]
    BotProvider,
    #[serde(rename = "botRecognitionFailureReason")]
    BotRecognitionFailureReason,
    #[serde(rename = "botResult")]
    BotResult,
    #[serde(rename = "botSessionId")]
    BotSessionId,
    #[serde(rename = "botSlot")]
    BotSlot,
    #[serde(rename = "botVersion")]
    BotVersion,
    #[serde(rename = "conversationId")]
    ConversationId,
    #[serde(rename = "externalContactId")]
    ExternalContactId,
    #[serde(rename = "knowledgeBaseId")]
    KnowledgeBaseId,
    #[serde(rename = "lastActionId")]
    LastActionId,
    #[serde(rename = "lastInputActionId")]
    LastInputActionId,
    #[serde(rename = "mediaType")]
    MediaType,
    #[serde(rename = "messageType")]
    MessageType,
    #[serde(rename = "selfServed")]
    SelfServed,
}

impl Default for GroupBy {
    fn default() -> GroupBy {
        Self::BotFinalIntent
    }
}
/// Behaves like a SQL SELECT clause. Only named metrics will be retrieved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metrics {
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

impl Default for Metrics {
    fn default() -> Metrics {
        Self::NBotSessions
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
