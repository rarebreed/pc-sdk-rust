/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// TextBotUserAgent : Information about the caller executing a bot flow.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TextBotUserAgent {
    /// The name of the user agent.
    #[serde(rename = "name")]
    pub name: Name,
}

impl TextBotUserAgent {
    /// Information about the caller executing a bot flow.
    pub fn new(name: Name) -> TextBotUserAgent {
        TextBotUserAgent {
            name,
        }
    }
}

/// The name of the user agent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
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

impl Default for Name {
    fn default() -> Name {
        Self::Phone
    }
}

