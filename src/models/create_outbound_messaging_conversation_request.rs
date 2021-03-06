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
pub struct CreateOutboundMessagingConversationRequest {
    /// The ID of the queue to be associated with the message. This will determine the fromAddress of the message.
    #[serde(rename = "queueId")]
    pub queue_id: String,
    /// The messaging address of the recipient of the message. For an SMS messenger type, the phone number address must be in E.164 format. E.g. +13175555555 or +34234234234
    #[serde(rename = "toAddress")]
    pub to_address: String,
    /// The messaging address messenger type.
    #[serde(rename = "toAddressMessengerType")]
    pub to_address_messenger_type: ToAddressMessengerType,
    /// An override to use an existing conversation.  If set to true, an existing conversation will be used if there is one within the conversation window.  If set to false, create request fails if there is a conversation within the conversation window.
    #[serde(rename = "useExistingConversation", skip_serializing_if = "Option::is_none")]
    pub use_existing_conversation: Option<bool>,
    /// The external contact with which the message will be associated.
    #[serde(rename = "externalContactId", skip_serializing_if = "Option::is_none")]
    pub external_contact_id: Option<String>,
}

impl CreateOutboundMessagingConversationRequest {
    pub fn new(queue_id: String, to_address: String, to_address_messenger_type: ToAddressMessengerType) -> CreateOutboundMessagingConversationRequest {
        CreateOutboundMessagingConversationRequest {
            queue_id,
            to_address,
            to_address_messenger_type,
            use_existing_conversation: None,
            external_contact_id: None,
        }
    }
}

/// The messaging address messenger type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ToAddressMessengerType {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "webmessaging")]
    Webmessaging,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "open")]
    Open,
}

impl Default for ToAddressMessengerType {
    fn default() -> ToAddressMessengerType {
        Self::Sms
    }
}

