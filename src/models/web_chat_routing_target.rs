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
pub struct WebChatRoutingTarget {
    /// The target type of the routing target, such as 'QUEUE'.
    #[serde(rename = "targetType")]
    pub target_type: TargetType,
    /// The target of the route, in the format appropriate given the 'targetType'.
    #[serde(rename = "targetAddress")]
    pub target_address: String,
    /// The list of skill names to use for routing.
    #[serde(rename = "skills", skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    /// The language name to use for routing.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The priority to assign to the conversation for routing.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}

impl WebChatRoutingTarget {
    pub fn new(target_type: TargetType, target_address: String) -> WebChatRoutingTarget {
        WebChatRoutingTarget {
            target_type,
            target_address,
            skills: None,
            language: None,
            priority: None,
        }
    }
}

/// The target type of the routing target, such as 'QUEUE'.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "QUEUE")]
    QUEUE,
}

impl Default for TargetType {
    fn default() -> TargetType {
        Self::QUEUE
    }
}

