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
pub struct SupportedLanguage {
    /// Architect supported language tag, e.g. en-us, es-us
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Whether or not this language is the default language
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl SupportedLanguage {
    pub fn new() -> SupportedLanguage {
        SupportedLanguage {
            language: None,
            is_default: None,
        }
    }
}


