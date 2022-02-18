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
pub struct PostTextResponse {
    /// The state of the bot after completion of the request
    #[serde(rename = "botState")]
    pub bot_state: BotState,
    /// The list of messages to respond with, if any
    #[serde(rename = "replyMessages", skip_serializing_if = "Option::is_none")]
    pub reply_messages: Option<Vec<crate::models::PostTextMessage>>,
    /// The name of the intent the bot is either processing or has processed, this will be blank if no intent could be detected.
    #[serde(rename = "intentName", skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    /// Data parameters detected and filled by the bot.
    #[serde(rename = "slots", skip_serializing_if = "Option::is_none")]
    pub slots: Option<::std::collections::HashMap<String, String>>,
    /// The optional ID specified in the request
    #[serde(rename = "botCorrelationId", skip_serializing_if = "Option::is_none")]
    pub bot_correlation_id: Option<String>,
    /// Raw data response from AWS (if called)
    #[serde(rename = "amazonLex", skip_serializing_if = "Option::is_none")]
    pub amazon_lex: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// Raw data response from Google Dialogflow (if called)
    #[serde(rename = "googleDialogFlow", skip_serializing_if = "Option::is_none")]
    pub google_dialog_flow: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// Raw data response from Genesys' Dialogengine (if called)
    #[serde(rename = "genesysDialogEngine", skip_serializing_if = "Option::is_none")]
    pub genesys_dialog_engine: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// Raw data response from Genesys' BotConnector (if called)
    #[serde(rename = "genesysBotConnector", skip_serializing_if = "Option::is_none")]
    pub genesys_bot_connector: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// Raw data response from Nuance Mix Dlg (if called)
    #[serde(rename = "nuanceMixDlg", skip_serializing_if = "Option::is_none")]
    pub nuance_mix_dlg: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PostTextResponse {
    pub fn new(bot_state: BotState) -> PostTextResponse {
        PostTextResponse {
            bot_state,
            reply_messages: None,
            intent_name: None,
            slots: None,
            bot_correlation_id: None,
            amazon_lex: None,
            google_dialog_flow: None,
            genesys_dialog_engine: None,
            genesys_bot_connector: None,
            nuance_mix_dlg: None,
        }
    }
}

/// The state of the bot after completion of the request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BotState {
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "MoreData")]
    MoreData,
}

impl Default for BotState {
    fn default() -> BotState {
        Self::Complete
    }
}
