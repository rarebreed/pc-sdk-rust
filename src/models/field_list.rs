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
pub struct FieldList {
    #[serde(rename = "customLabels", skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<bool>,
    #[serde(rename = "instructionText", skip_serializing_if = "Option::is_none")]
    pub instruction_text: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "labelKeys", skip_serializing_if = "Option::is_none")]
    pub label_keys: Option<Vec<String>>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "repeatable", skip_serializing_if = "Option::is_none")]
    pub repeatable: Option<bool>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "gdpr", skip_serializing_if = "Option::is_none")]
    pub gdpr: Option<bool>,
}

impl FieldList {
    pub fn new() -> FieldList {
        FieldList {
            custom_labels: None,
            instruction_text: None,
            key: None,
            label_keys: None,
            params: None,
            repeatable: None,
            state: None,
            _type: None,
            required: None,
            gdpr: None,
        }
    }
}


