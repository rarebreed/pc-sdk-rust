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
pub struct IpAddressRange {
    #[serde(rename = "cidr", skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl IpAddressRange {
    pub fn new() -> IpAddressRange {
        IpAddressRange {
            cidr: None,
            service: None,
            region: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Service {
    #[serde(rename = "data-actions")]
    DataActions,
    #[serde(rename = "smtp")]
    Smtp,
}

impl Default for Service {
    fn default() -> Service {
        Self::DataActions
    }
}
