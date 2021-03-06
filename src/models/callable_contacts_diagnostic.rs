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
pub struct CallableContactsDiagnostic {
    #[serde(rename = "attemptLimits", skip_serializing_if = "Option::is_none")]
    pub attempt_limits: Option<Box<crate::models::DomainEntityRef>>,
    /// Do not call lists for the campaign
    #[serde(rename = "dncLists", skip_serializing_if = "Option::is_none")]
    pub dnc_lists: Option<Vec<crate::models::DomainEntityRef>>,
    #[serde(rename = "callableTimeSet", skip_serializing_if = "Option::is_none")]
    pub callable_time_set: Option<Box<crate::models::DomainEntityRef>>,
    /// Rule sets for the campaign
    #[serde(rename = "ruleSets", skip_serializing_if = "Option::is_none")]
    pub rule_sets: Option<Vec<crate::models::DomainEntityRef>>,
}

impl CallableContactsDiagnostic {
    pub fn new() -> CallableContactsDiagnostic {
        CallableContactsDiagnostic {
            attempt_limits: None,
            dnc_lists: None,
            callable_time_set: None,
            rule_sets: None,
        }
    }
}


