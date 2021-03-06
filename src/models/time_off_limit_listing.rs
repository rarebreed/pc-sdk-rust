/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// TimeOffLimitListing : The list of time off limit objects



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeOffLimitListing {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::TimeOffLimit>>,
}

impl TimeOffLimitListing {
    /// The list of time off limit objects
    pub fn new() -> TimeOffLimitListing {
        TimeOffLimitListing {
            entities: None,
        }
    }
}


