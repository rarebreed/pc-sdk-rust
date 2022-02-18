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
pub struct ConversationMetrics {
    #[serde(rename = "conversation", skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Box<crate::models::AddressableEntityRef>>,
    /// The Sentiment Score
    #[serde(rename = "sentimentScore", skip_serializing_if = "Option::is_none")]
    pub sentiment_score: Option<f64>,
    /// The Sentiment Trend
    #[serde(rename = "sentimentTrend", skip_serializing_if = "Option::is_none")]
    pub sentiment_trend: Option<f64>,
    /// The Sentiment Trend Class
    #[serde(rename = "sentimentTrendClass", skip_serializing_if = "Option::is_none")]
    pub sentiment_trend_class: Option<SentimentTrendClass>,
    #[serde(rename = "participantMetrics", skip_serializing_if = "Option::is_none")]
    pub participant_metrics: Option<Box<crate::models::ParticipantMetrics>>,
}

impl ConversationMetrics {
    pub fn new() -> ConversationMetrics {
        ConversationMetrics {
            conversation: None,
            sentiment_score: None,
            sentiment_trend: None,
            sentiment_trend_class: None,
            participant_metrics: None,
        }
    }
}

/// The Sentiment Trend Class
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SentimentTrendClass {
    #[serde(rename = "NotCalculated")]
    NotCalculated,
    #[serde(rename = "Declining")]
    Declining,
    #[serde(rename = "SlightlyDeclining")]
    SlightlyDeclining,
    #[serde(rename = "NoChange")]
    NoChange,
    #[serde(rename = "SlightlyImproving")]
    SlightlyImproving,
    #[serde(rename = "Improving")]
    Improving,
}

impl Default for SentimentTrendClass {
    fn default() -> SentimentTrendClass {
        Self::NotCalculated
    }
}

