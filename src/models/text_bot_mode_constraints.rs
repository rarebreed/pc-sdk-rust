/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// TextBotModeConstraints : Mode constraints to observe when operating on a bot flow.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TextBotModeConstraints {
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<crate::models::TextBotTextModeConstraints>>,
}

impl TextBotModeConstraints {
    /// Mode constraints to observe when operating on a bot flow.
    pub fn new() -> TextBotModeConstraints {
        TextBotModeConstraints {
            text: None,
        }
    }
}


