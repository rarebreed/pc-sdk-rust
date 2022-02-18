/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// EmergencyCallFlow : An emergency flow associates a call flow to use in an emergency with the ivr(s) to route to it.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmergencyCallFlow {
    #[serde(rename = "emergencyFlow", skip_serializing_if = "Option::is_none")]
    pub emergency_flow: Option<Box<crate::models::DomainEntityRef>>,
    /// The IVR(s) to route to the call flow during an emergency.
    #[serde(rename = "ivrs", skip_serializing_if = "Option::is_none")]
    pub ivrs: Option<Vec<crate::models::DomainEntityRef>>,
}

impl EmergencyCallFlow {
    /// An emergency flow associates a call flow to use in an emergency with the ivr(s) to route to it.
    pub fn new() -> EmergencyCallFlow {
        EmergencyCallFlow {
            emergency_flow: None,
            ivrs: None,
        }
    }
}


