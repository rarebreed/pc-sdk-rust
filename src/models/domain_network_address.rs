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
pub struct DomainNetworkAddress {
    /// The type of address.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// An IPv4 or IPv6 IP address. When specifying an address of type \"ip\", use CIDR format for the subnet mask.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// True if this address will persist on Edge restart.  Addresses assigned by DHCP will be returned as false.
    #[serde(rename = "persistent", skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    /// The address family for this address.
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<Family>,
}

impl DomainNetworkAddress {
    pub fn new() -> DomainNetworkAddress {
        DomainNetworkAddress {
            _type: None,
            address: None,
            persistent: None,
            family: None,
        }
    }
}

/// The type of address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "dns")]
    Dns,
    #[serde(rename = "gateway")]
    Gateway,
    #[serde(rename = "tdm")]
    Tdm,
}

impl Default for Type {
    fn default() -> Type {
        Self::Ip
    }
}
/// The address family for this address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Family {
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "23")]
    _23,
}

impl Default for Family {
    fn default() -> Family {
        Self::_2
    }
}

