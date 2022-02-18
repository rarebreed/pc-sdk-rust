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
pub struct ReplacementTerm {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "existingValue", skip_serializing_if = "Option::is_none")]
    pub existing_value: Option<String>,
    #[serde(rename = "updatedValue", skip_serializing_if = "Option::is_none")]
    pub updated_value: Option<String>,
}

impl ReplacementTerm {
    pub fn new() -> ReplacementTerm {
        ReplacementTerm {
            _type: None,
            existing_value: None,
            updated_value: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "NAME")]
    NAME,
    #[serde(rename = "ADDRESS")]
    ADDRESS,
    #[serde(rename = "PHONE")]
    PHONE,
    #[serde(rename = "EMAIL")]
    EMAIL,
    #[serde(rename = "TWITTER")]
    TWITTER,
}

impl Default for Type {
    fn default() -> Type {
        Self::NAME
    }
}

