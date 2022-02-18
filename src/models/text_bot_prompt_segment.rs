/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// TextBotPromptSegment : Data for a single bot flow prompt segment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TextBotPromptSegment {
    /// The text of this prompt segment.
    #[serde(rename = "text")]
    pub text: String,
    /// The segment type which describes any semantics about the 'text' and also indicates which other field might include additional relevant info.
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Box<crate::models::Format>>,
    /// Details to display Rich Media content. This is only populated when the segment 'type' is 'Rich Media'.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<crate::models::MessageContent>>,
}

impl TextBotPromptSegment {
    /// Data for a single bot flow prompt segment.
    pub fn new(text: String, _type: Type) -> TextBotPromptSegment {
        TextBotPromptSegment {
            text,
            _type,
            format: None,
            content: None,
        }
    }
}

/// The segment type which describes any semantics about the 'text' and also indicates which other field might include additional relevant info.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "RichMedia")]
    RichMedia,
}

impl Default for Type {
    fn default() -> Type {
        Self::Text
    }
}
