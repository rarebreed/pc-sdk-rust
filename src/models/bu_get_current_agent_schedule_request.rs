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
pub struct BuGetCurrentAgentScheduleRequest {
    /// Start date of the range to search. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// End date of the range to search. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "endDate")]
    pub end_date: String,
}

impl BuGetCurrentAgentScheduleRequest {
    pub fn new(start_date: String, end_date: String) -> BuGetCurrentAgentScheduleRequest {
        BuGetCurrentAgentScheduleRequest {
            start_date,
            end_date,
        }
    }
}

