/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// BotConnectorBotVersion : A version description for a botConnector bot.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotConnectorBotVersion {
    /// The name of the version. This can be up to 100 characters long and must be comprised of displayable characters without leading or trailing whitespace
    #[serde(rename = "version")]
    pub version: String,
    /// The supported languages for this bot. EG 'en-us' or 'es', etc; These language codes are W3C language identification tags (ISO 639-1 for the language name and ISO 3166 for the country code)
    #[serde(rename = "supportedLanguages")]
    pub supported_languages: Vec<String>,
    /// A list of potential intents this bot will return, limit of 50
    #[serde(rename = "intents")]
    pub intents: Vec<crate::models::BotIntent>,
}

impl BotConnectorBotVersion {
    /// A version description for a botConnector bot.
    pub fn new(version: String, supported_languages: Vec<String>, intents: Vec<crate::models::BotIntent>) -> BotConnectorBotVersion {
        BotConnectorBotVersion {
            version,
            supported_languages,
            intents,
        }
    }
}

