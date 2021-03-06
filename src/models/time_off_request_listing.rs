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
pub struct TimeOffRequestListing {
    /// List of time off requests
    #[serde(rename = "entities")]
    pub entities: Vec<crate::models::TimeOffRequest>,
}

impl TimeOffRequestListing {
    pub fn new(entities: Vec<crate::models::TimeOffRequest>) -> TimeOffRequestListing {
        TimeOffRequestListing {
            entities,
        }
    }
}


