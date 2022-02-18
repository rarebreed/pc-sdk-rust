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
pub struct SurveyQuestion {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "helpText", skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "naEnabled", skip_serializing_if = "Option::is_none")]
    pub na_enabled: Option<bool>,
    #[serde(rename = "visibilityCondition", skip_serializing_if = "Option::is_none")]
    pub visibility_condition: Option<Box<crate::models::VisibilityCondition>>,
    /// Options from which to choose an answer for this question. Only used by Multiple Choice type questions.
    #[serde(rename = "answerOptions", skip_serializing_if = "Option::is_none")]
    pub answer_options: Option<Vec<crate::models::AnswerOption>>,
    /// How many characters are allowed in the text response to this question. Used by NPS and Free Text question types.
    #[serde(rename = "maxResponseCharacters", skip_serializing_if = "Option::is_none")]
    pub max_response_characters: Option<i32>,
    /// Prompt for details explaining the chosen NPS score. Used by NPS questions.
    #[serde(rename = "explanationPrompt", skip_serializing_if = "Option::is_none")]
    pub explanation_prompt: Option<String>,
}

impl SurveyQuestion {
    pub fn new() -> SurveyQuestion {
        SurveyQuestion {
            id: None,
            text: None,
            help_text: None,
            _type: None,
            na_enabled: None,
            visibility_condition: None,
            answer_options: None,
            max_response_characters: None,
            explanation_prompt: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "multipleChoiceQuestion")]
    MultipleChoiceQuestion,
    #[serde(rename = "freeTextQuestion")]
    FreeTextQuestion,
    #[serde(rename = "npsQuestion")]
    NpsQuestion,
    #[serde(rename = "readOnlyTextBlockQuestion")]
    ReadOnlyTextBlockQuestion,
}

impl Default for Type {
    fn default() -> Type {
        Self::MultipleChoiceQuestion
    }
}
