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
pub struct UserConversationSummary {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "call", skip_serializing_if = "Option::is_none")]
    pub call: Option<Box<crate::models::MediaSummary>>,
    #[serde(rename = "callback", skip_serializing_if = "Option::is_none")]
    pub callback: Option<Box<crate::models::MediaSummary>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<crate::models::MediaSummary>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<crate::models::MediaSummary>>,
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<Box<crate::models::MediaSummary>>,
    #[serde(rename = "socialExpression", skip_serializing_if = "Option::is_none")]
    pub social_expression: Option<Box<crate::models::MediaSummary>>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<crate::models::MediaSummary>>,
}

impl UserConversationSummary {
    pub fn new() -> UserConversationSummary {
        UserConversationSummary {
            user_id: None,
            call: None,
            callback: None,
            email: None,
            message: None,
            chat: None,
            social_expression: None,
            video: None,
        }
    }
}


