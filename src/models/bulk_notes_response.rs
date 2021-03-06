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
pub struct BulkNotesResponse {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::BulkResponseResultNoteNote>>,
    #[serde(rename = "errorCount", skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "errorIndexes", skip_serializing_if = "Option::is_none")]
    pub error_indexes: Option<Vec<i32>>,
}

impl BulkNotesResponse {
    pub fn new() -> BulkNotesResponse {
        BulkNotesResponse {
            results: None,
            error_count: None,
            error_indexes: None,
        }
    }
}


