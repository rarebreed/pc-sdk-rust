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
pub struct AsyncConversationQuery {
    /// Filters that target conversation-level data
    #[serde(rename = "conversationFilters", skip_serializing_if = "Option::is_none")]
    pub conversation_filters: Option<Vec<crate::models::ConversationDetailQueryFilter>>,
    /// Filters that target individual segments within a conversation
    #[serde(rename = "segmentFilters", skip_serializing_if = "Option::is_none")]
    pub segment_filters: Option<Vec<crate::models::SegmentDetailQueryFilter>>,
    /// Filters that target evaluations
    #[serde(rename = "evaluationFilters", skip_serializing_if = "Option::is_none")]
    pub evaluation_filters: Option<Vec<crate::models::EvaluationDetailQueryFilter>>,
    /// Filters that target surveys
    #[serde(rename = "surveyFilters", skip_serializing_if = "Option::is_none")]
    pub survey_filters: Option<Vec<crate::models::SurveyDetailQueryFilter>>,
    /// Filters that target resolutions
    #[serde(rename = "resolutionFilters", skip_serializing_if = "Option::is_none")]
    pub resolution_filters: Option<Vec<crate::models::ResolutionDetailQueryFilter>>,
    /// Sort the result set in ascending/descending order. Default is ascending
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Specify which data element within the result set to use for sorting. The options  to use as a basis for sorting the results: conversationStart, segmentStart, and segmentEnd. If not specified, the default is conversationStart
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<OrderBy>,
    /// Specifies the date and time range of data being queried. Results will include all conversations that had activity during the interval. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss
    #[serde(rename = "interval")]
    pub interval: String,
    /// Specify number of results to be returned
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Add a filter to only include conversations that started after the beginning of the interval start date (UTC)
    #[serde(rename = "startOfDayIntervalMatching", skip_serializing_if = "Option::is_none")]
    pub start_of_day_interval_matching: Option<bool>,
}

impl AsyncConversationQuery {
    pub fn new(interval: String) -> AsyncConversationQuery {
        AsyncConversationQuery {
            conversation_filters: None,
            segment_filters: None,
            evaluation_filters: None,
            survey_filters: None,
            resolution_filters: None,
            order: None,
            order_by: None,
            interval,
            limit: None,
            start_of_day_interval_matching: None,
        }
    }
}

/// Sort the result set in ascending/descending order. Default is ascending
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Order {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for Order {
    fn default() -> Order {
        Self::Asc
    }
}
/// Specify which data element within the result set to use for sorting. The options  to use as a basis for sorting the results: conversationStart, segmentStart, and segmentEnd. If not specified, the default is conversationStart
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderBy {
    #[serde(rename = "conversationStart")]
    ConversationStart,
    #[serde(rename = "conversationEnd")]
    ConversationEnd,
    #[serde(rename = "segmentStart")]
    SegmentStart,
    #[serde(rename = "segmentEnd")]
    SegmentEnd,
}

impl Default for OrderBy {
    fn default() -> OrderBy {
        Self::ConversationStart
    }
}

