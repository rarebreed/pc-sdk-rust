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
pub struct WorkspaceSummary {
    #[serde(rename = "totalDocumentCount", skip_serializing_if = "Option::is_none")]
    pub total_document_count: Option<i64>,
    #[serde(rename = "totalDocumentByteCount", skip_serializing_if = "Option::is_none")]
    pub total_document_byte_count: Option<i64>,
}

impl WorkspaceSummary {
    pub fn new() -> WorkspaceSummary {
        WorkspaceSummary {
            total_document_count: None,
            total_document_byte_count: None,
        }
    }
}


