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
pub struct AdditionalMessage {
    /// The body of the text message.  Maximum character counts are: SMS - 765 characters, other channels - 2000 characters.
    #[serde(rename = "textBody")]
    pub text_body: String,
    /// The media ids associated with the text message. See https://developer.genesys.cloud/api/rest/v2/conversations/messaging-media-upload for example usage.
    #[serde(rename = "mediaIds", skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,
    /// The sticker ids associated with the text message.
    #[serde(rename = "stickerIds", skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<String>>,
    #[serde(rename = "messagingTemplate", skip_serializing_if = "Option::is_none")]
    pub messaging_template: Option<Box<crate::models::MessagingTemplateRequest>>,
}

impl AdditionalMessage {
    pub fn new(text_body: String) -> AdditionalMessage {
        AdditionalMessage {
            text_body,
            media_ids: None,
            sticker_ids: None,
            messaging_template: None,
        }
    }
}


