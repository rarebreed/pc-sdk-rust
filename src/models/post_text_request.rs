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
pub struct PostTextRequest {
    /// ID of the bot to send the text to.
    #[serde(rename = "botId")]
    pub bot_id: String,
    /// Alias/Version of the bot
    #[serde(rename = "botAlias", skip_serializing_if = "Option::is_none")]
    pub bot_alias: Option<String>,
    /// the integration service id for the bot's credentials
    #[serde(rename = "integrationId")]
    pub integration_id: String,
    /// GUID for this bot's session
    #[serde(rename = "botSessionId")]
    pub bot_session_id: String,
    #[serde(rename = "postTextMessage")]
    pub post_text_message: Box<crate::models::PostTextMessage>,
    /// The launguage code the bot will run under
    #[serde(rename = "languageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Override timeout for the bot session. This should be greater than 10 minutes.
    #[serde(rename = "botSessionTimeoutMinutes", skip_serializing_if = "Option::is_none")]
    pub bot_session_timeout_minutes: Option<i32>,
    /// The channels this bot is utilizing
    #[serde(rename = "botChannels", skip_serializing_if = "Option::is_none")]
    pub bot_channels: Option<Vec<BotChannels>>,
    /// Id for tracking the activity - this will be returned in the response
    #[serde(rename = "botCorrelationId", skip_serializing_if = "Option::is_none")]
    pub bot_correlation_id: Option<String>,
    /// If the channels list contains a 'Messaging' item and the messaging platform is known, include it here to get accurate analytics
    #[serde(rename = "messagingPlatformType", skip_serializing_if = "Option::is_none")]
    pub messaging_platform_type: Option<MessagingPlatformType>,
    #[serde(rename = "amazonLexRequest", skip_serializing_if = "Option::is_none")]
    pub amazon_lex_request: Option<Box<crate::models::AmazonLexRequest>>,
    #[serde(rename = "googleDialogflow", skip_serializing_if = "Option::is_none")]
    pub google_dialogflow: Option<Box<crate::models::GoogleDialogflowCustomSettings>>,
    #[serde(rename = "genesysBotConnector", skip_serializing_if = "Option::is_none")]
    pub genesys_bot_connector: Option<Box<crate::models::GenesysBotConnector>>,
    #[serde(rename = "nuanceMixDlg", skip_serializing_if = "Option::is_none")]
    pub nuance_mix_dlg: Option<Box<crate::models::NuanceMixDlgSettings>>,
}

impl PostTextRequest {
    pub fn new(bot_id: String, integration_id: String, bot_session_id: String, post_text_message: crate::models::PostTextMessage) -> PostTextRequest {
        PostTextRequest {
            bot_id,
            bot_alias: None,
            integration_id,
            bot_session_id,
            post_text_message: Box::new(post_text_message),
            language_code: None,
            bot_session_timeout_minutes: None,
            bot_channels: None,
            bot_correlation_id: None,
            messaging_platform_type: None,
            amazon_lex_request: None,
            google_dialogflow: None,
            genesys_bot_connector: None,
            nuance_mix_dlg: None,
        }
    }
}

/// The channels this bot is utilizing
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BotChannels {
    #[serde(rename = "Call")]
    Call,
    #[serde(rename = "Callback")]
    Callback,
    #[serde(rename = "Messaging")]
    Messaging,
    #[serde(rename = "Webchat")]
    Webchat,
}

impl Default for BotChannels {
    fn default() -> BotChannels {
        Self::Call
    }
}
/// If the channels list contains a 'Messaging' item and the messaging platform is known, include it here to get accurate analytics
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessagingPlatformType {
    #[serde(rename = "Phone")]
    Phone,
    #[serde(rename = "SMS")]
    SMS,
    #[serde(rename = "GenesysWebWidget")]
    GenesysWebWidget,
    #[serde(rename = "FacebookMessenger")]
    FacebookMessenger,
    #[serde(rename = "WeChat")]
    WeChat,
    #[serde(rename = "Whatsapp")]
    Whatsapp,
    #[serde(rename = "AppleBusinessChat")]
    AppleBusinessChat,
    #[serde(rename = "Telegram")]
    Telegram,
    #[serde(rename = "Slack")]
    Slack,
    #[serde(rename = "Signal")]
    Signal,
    #[serde(rename = "Line")]
    Line,
    #[serde(rename = "Discord")]
    Discord,
    #[serde(rename = "TwitterDirectMessage")]
    TwitterDirectMessage,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for MessagingPlatformType {
    fn default() -> MessagingPlatformType {
        Self::Phone
    }
}

