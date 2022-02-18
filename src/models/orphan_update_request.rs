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
pub struct OrphanUpdateRequest {
    /// The orphan recording's archive date. Must be greater than 1 day from now if set. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "archiveDate", skip_serializing_if = "Option::is_none")]
    pub archive_date: Option<String>,
    /// The orphan recording's delete date. Must be greater than archiveDate and exportDate if set, otherwise one day from now. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "deleteDate", skip_serializing_if = "Option::is_none")]
    pub delete_date: Option<String>,
    /// The orphan recording's export date. Must be greater than 1 day from now if set. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "exportDate", skip_serializing_if = "Option::is_none")]
    pub export_date: Option<String>,
    /// IntegrationId to access AWS S3 bucket for export. This field is required if exportDate is set.
    #[serde(rename = "integrationId", skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    /// A conversation Id that this orphan's recording is to be attached to. If not present, the conversationId will be deduced from the recording media.
    #[serde(rename = "conversationId", skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

impl OrphanUpdateRequest {
    pub fn new() -> OrphanUpdateRequest {
        OrphanUpdateRequest {
            archive_date: None,
            delete_date: None,
            export_date: None,
            integration_id: None,
            conversation_id: None,
        }
    }
}

