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
pub struct Section {
    #[serde(rename = "fieldList", skip_serializing_if = "Option::is_none")]
    pub field_list: Option<Vec<crate::models::FieldList>>,
    #[serde(rename = "instructionText", skip_serializing_if = "Option::is_none")]
    pub instruction_text: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl Section {
    pub fn new() -> Section {
        Section {
            field_list: None,
            instruction_text: None,
            key: None,
            state: None,
        }
    }
}


