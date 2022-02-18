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
pub struct BuCopyScheduleRequest {
    /// The description for the new schedule
    #[serde(rename = "description")]
    pub description: String,
    /// The start weekDate for the new copy of the schedule. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "weekDate")]
    pub week_date: String,
}

impl BuCopyScheduleRequest {
    pub fn new(description: String, week_date: String) -> BuCopyScheduleRequest {
        BuCopyScheduleRequest {
            description,
            week_date,
        }
    }
}


