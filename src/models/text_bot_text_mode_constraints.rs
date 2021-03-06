/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// TextBotTextModeConstraints : Mode constraints to observe when operating on a bot flow.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TextBotTextModeConstraints {
    /// The list of language preferences by their ISO language code.
    #[serde(rename = "languagePreferences")]
    pub language_preferences: Vec<String>,
}

impl TextBotTextModeConstraints {
    /// Mode constraints to observe when operating on a bot flow.
    pub fn new(language_preferences: Vec<String>) -> TextBotTextModeConstraints {
        TextBotTextModeConstraints {
            language_preferences,
        }
    }
}


