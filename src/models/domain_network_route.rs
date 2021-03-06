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
pub struct DomainNetworkRoute {
    /// The IPv4 or IPv6 route prefix in CIDR notation.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The IPv4 or IPv6 nexthop IP address.
    #[serde(rename = "nexthop", skip_serializing_if = "Option::is_none")]
    pub nexthop: Option<String>,
    /// True if this route will persist on Edge restart.  Routes assigned by DHCP will be returned as false.
    #[serde(rename = "persistent", skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    /// The metric being used for route. Lower values will have a higher priority.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<i32>,
    /// The address family for this route.
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<Family>,
}

impl DomainNetworkRoute {
    pub fn new() -> DomainNetworkRoute {
        DomainNetworkRoute {
            prefix: None,
            nexthop: None,
            persistent: None,
            metric: None,
            family: None,
        }
    }
}

/// The address family for this route.
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

