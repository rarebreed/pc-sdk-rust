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
pub struct PatchSurveyQuestion {
    /// Type of survey question.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Label of question.
    #[serde(rename = "label")]
    pub label: String,
    /// The customer property that the answer maps to.
    #[serde(rename = "customerProperty", skip_serializing_if = "Option::is_none")]
    pub customer_property: Option<CustomerProperty>,
    /// Choices available to user.
    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<String>>,
    /// Whether answering this question is mandatory.
    #[serde(rename = "isMandatory", skip_serializing_if = "Option::is_none")]
    pub is_mandatory: Option<bool>,
}

impl PatchSurveyQuestion {
    pub fn new(label: String) -> PatchSurveyQuestion {
        PatchSurveyQuestion {
            _type: None,
            label,
            customer_property: None,
            choices: None,
            is_mandatory: None,
        }
    }
}

/// Type of survey question.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "textarea")]
    Textarea,
}

impl Default for Type {
    fn default() -> Type {
        Self::Text
    }
}
/// The customer property that the answer maps to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CustomerProperty {
    #[serde(rename = "givenName")]
    GivenName,
    #[serde(rename = "familyName")]
    FamilyName,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "gender")]
    Gender,
    #[serde(rename = "companyName")]
    CompanyName,
}

impl Default for CustomerProperty {
    fn default() -> CustomerProperty {
        Self::GivenName
    }
}

