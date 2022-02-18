/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// MessageContent : Message content element.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessageContent {
    /// Type of this content element. If contentType = \"Attachment\" only one item is allowed.
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<crate::models::ContentLocation>>,
    #[serde(rename = "attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Box<crate::models::ContentAttachment>>,
    #[serde(rename = "quickReply", skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<Box<crate::models::ContentQuickReply>>,
    #[serde(rename = "buttonResponse", skip_serializing_if = "Option::is_none")]
    pub button_response: Option<Box<crate::models::ContentButtonResponse>>,
    #[serde(rename = "generic", skip_serializing_if = "Option::is_none")]
    pub generic: Option<Box<crate::models::ContentGeneric>>,
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<Box<crate::models::ContentList>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<crate::models::ContentNotificationTemplate>>,
    /// A set of reactions to a message.
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<crate::models::ContentReaction>>,
    #[serde(rename = "mention", skip_serializing_if = "Option::is_none")]
    pub mention: Option<Box<crate::models::MessagingRecipient>>,
    #[serde(rename = "postback", skip_serializing_if = "Option::is_none")]
    pub postback: Option<Box<crate::models::ContentPostback>>,
}

impl MessageContent {
    /// Message content element.
    pub fn new(content_type: ContentType) -> MessageContent {
        MessageContent {
            content_type,
            location: None,
            attachment: None,
            quick_reply: None,
            button_response: None,
            generic: None,
            list: None,
            template: None,
            reactions: None,
            mention: None,
            postback: None,
        }
    }
}

/// Type of this content element. If contentType = \"Attachment\" only one item is allowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "Attachment")]
    Attachment,
    #[serde(rename = "Location")]
    Location,
    #[serde(rename = "QuickReply")]
    QuickReply,
    #[serde(rename = "Notification")]
    Notification,
    #[serde(rename = "GenericTemplate")]
    GenericTemplate,
    #[serde(rename = "ListTemplate")]
    ListTemplate,
    #[serde(rename = "Postback")]
    Postback,
    #[serde(rename = "Reactions")]
    Reactions,
    #[serde(rename = "Mention")]
    Mention,
    #[serde(rename = "ButtonResponse")]
    ButtonResponse,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Attachment
    }
}

