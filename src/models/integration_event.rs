/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// IntegrationEvent : Describes an event that has happened related to an integration



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IntegrationEvent {
    /// Unique ID for this event
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// Correlation ID for the event
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// Time the event occurred. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Indicates the severity of the event.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    /// A classification for the event. Suitable for programmatic searching, sorting, or filtering
    #[serde(rename = "eventCode", skip_serializing_if = "Option::is_none")]
    pub event_code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<crate::models::MessageInfo>>,
    /// Collection of entities affected by or pertaining to the event (e.g. a list of Integrations or Bridge connectors)
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::EventEntity>>,
    /// Map of context attributes specific to this event.
    #[serde(rename = "contextAttributes", skip_serializing_if = "Option::is_none")]
    pub context_attributes: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "detailMessage", skip_serializing_if = "Option::is_none")]
    pub detail_message: Option<Box<crate::models::MessageInfo>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
}

impl IntegrationEvent {
    /// Describes an event that has happened related to an integration
    pub fn new() -> IntegrationEvent {
        IntegrationEvent {
            id: None,
            self_uri: None,
            correlation_id: None,
            timestamp: None,
            level: None,
            event_code: None,
            message: None,
            entities: None,
            context_attributes: None,
            detail_message: None,
            user: None,
        }
    }
}

/// Indicates the severity of the event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "INFO")]
    INFO,
    #[serde(rename = "WARN")]
    WARN,
    #[serde(rename = "ERROR")]
    ERROR,
    #[serde(rename = "CRITICAL")]
    CRITICAL,
}

impl Default for Level {
    fn default() -> Level {
        Self::INFO
    }
}

