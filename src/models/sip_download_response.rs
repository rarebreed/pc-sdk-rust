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
pub struct SipDownloadResponse {
    /// unique id of the downloaded file
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    pub download_id: Option<String>,
    /// Document id of pcap file
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
}

impl SipDownloadResponse {
    pub fn new() -> SipDownloadResponse {
        SipDownloadResponse {
            download_id: None,
            document_id: None,
        }
    }
}


