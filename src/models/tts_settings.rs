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
pub struct TtsSettings {
    /// ID of the global default TTS engine
    #[serde(rename = "defaultEngine")]
    pub default_engine: String,
    /// The list of default overrides for specific languages
    #[serde(rename = "languageOverrides")]
    pub language_overrides: Vec<crate::models::LanguageOverride>,
}

impl TtsSettings {
    pub fn new(default_engine: String, language_overrides: Vec<crate::models::LanguageOverride>) -> TtsSettings {
        TtsSettings {
            default_engine,
            language_overrides,
        }
    }
}


