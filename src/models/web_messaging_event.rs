/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// WebMessagingEvent : Message event element.  Examples include: system messages, typing indicators, cobrowse offerings.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebMessagingEvent {
    /// Type of this event element
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    #[serde(rename = "coBrowse", skip_serializing_if = "Option::is_none")]
    pub co_browse: Option<Box<crate::models::WebMessagingEventCoBrowse>>,
}

impl WebMessagingEvent {
    /// Message event element.  Examples include: system messages, typing indicators, cobrowse offerings.
    pub fn new(event_type: EventType) -> WebMessagingEvent {
        WebMessagingEvent {
            event_type,
            co_browse: None,
        }
    }
}

/// Type of this event element
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "CoBrowse")]
    CoBrowse,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::CoBrowse
    }
}

