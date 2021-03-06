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
pub struct RoutePathResponse {
    #[serde(rename = "queue", skip_serializing_if = "Option::is_none")]
    pub queue: Option<Box<crate::models::QueueReference>>,
    /// The media type of the given queue associated with the route path
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<crate::models::LanguageReference>>,
    /// The set of skills associated with the route path
    #[serde(rename = "skills", skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<crate::models::RoutingSkillReference>>,
}

impl RoutePathResponse {
    pub fn new() -> RoutePathResponse {
        RoutePathResponse {
            queue: None,
            media_type: None,
            language: None,
            skills: None,
        }
    }
}

/// The media type of the given queue associated with the route path
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "Voice")]
    Voice,
    #[serde(rename = "Chat")]
    Chat,
    #[serde(rename = "Email")]
    Email,
    #[serde(rename = "Callback")]
    Callback,
    #[serde(rename = "Message")]
    Message,
}

impl Default for MediaType {
    fn default() -> MediaType {
        Self::Voice
    }
}

