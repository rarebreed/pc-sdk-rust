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
pub struct EdgeConnectionInfo {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Interface used for the connection on the edge
    #[serde(rename = "interfaceName", skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    /// IP address of the interface
    #[serde(rename = "interfaceIpAddress", skip_serializing_if = "Option::is_none")]
    pub interface_ip_address: Option<String>,
    /// Connection errors
    #[serde(rename = "connectionErrors", skip_serializing_if = "Option::is_none")]
    pub connection_errors: Option<Vec<String>>,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<Box<crate::models::AddressableEntityRef>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl EdgeConnectionInfo {
    pub fn new() -> EdgeConnectionInfo {
        EdgeConnectionInfo {
            id: None,
            name: None,
            interface_name: None,
            interface_ip_address: None,
            connection_errors: None,
            site: None,
            self_uri: None,
        }
    }
}

