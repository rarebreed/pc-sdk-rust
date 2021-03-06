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
pub struct TrunkBaseAssignment {
    /// The address family to use with the trunk base settings. 2=IPv4, 23=IPv6
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<i32>,
    #[serde(rename = "trunkBase", skip_serializing_if = "Option::is_none")]
    pub trunk_base: Option<Box<crate::models::TrunkBase>>,
}

impl TrunkBaseAssignment {
    pub fn new() -> TrunkBaseAssignment {
        TrunkBaseAssignment {
            family: None,
            trunk_base: None,
        }
    }
}


