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
pub struct QueueEmailAddress {
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Box<crate::models::InboundRoute>>,
}

impl QueueEmailAddress {
    pub fn new() -> QueueEmailAddress {
        QueueEmailAddress {
            domain: None,
            route: None,
        }
    }
}

