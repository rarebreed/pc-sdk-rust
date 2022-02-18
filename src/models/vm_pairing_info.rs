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
pub struct VmPairingInfo {
    #[serde(rename = "meta-data", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<Box<crate::models::MetaData>>,
    #[serde(rename = "edge-id", skip_serializing_if = "Option::is_none")]
    pub edge_id: Option<String>,
    #[serde(rename = "auth-token", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "org-id", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
}

impl VmPairingInfo {
    pub fn new() -> VmPairingInfo {
        VmPairingInfo {
            meta_data: None,
            edge_id: None,
            auth_token: None,
            org_id: None,
        }
    }
}

