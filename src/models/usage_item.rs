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
pub struct UsageItem {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "totalDocumentByteCount", skip_serializing_if = "Option::is_none")]
    pub total_document_byte_count: Option<i64>,
    #[serde(rename = "totalDocumentCount", skip_serializing_if = "Option::is_none")]
    pub total_document_count: Option<i64>,
}

impl UsageItem {
    pub fn new() -> UsageItem {
        UsageItem {
            _type: None,
            total_document_byte_count: None,
            total_document_count: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "RECORDING")]
    RECORDING,
    #[serde(rename = "FAX")]
    FAX,
    #[serde(rename = "DOCUMENT")]
    DOCUMENT,
    #[serde(rename = "ALL")]
    ALL,
}

impl Default for Type {
    fn default() -> Type {
        Self::RECORDING
    }
}

