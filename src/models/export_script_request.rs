/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ExportScriptRequest : Creating an exported script via Download Service



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExportScriptRequest {
    /// The final file name (no extension) of the script download: <fileName>.script
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The UUID version of the script to be exported.  Defaults to the current editable version.
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl ExportScriptRequest {
    /// Creating an exported script via Download Service
    pub fn new() -> ExportScriptRequest {
        ExportScriptRequest {
            file_name: None,
            version_id: None,
        }
    }
}


