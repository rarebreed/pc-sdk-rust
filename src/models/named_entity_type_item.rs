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
pub struct NamedEntityTypeItem {
    /// A value for an named entity type definition.
    #[serde(rename = "value")]
    pub value: String,
    /// Synonyms for the given named entity value.
    #[serde(rename = "synonyms", skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
}

impl NamedEntityTypeItem {
    pub fn new(value: String) -> NamedEntityTypeItem {
        NamedEntityTypeItem {
            value,
            synonyms: None,
        }
    }
}

