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
pub struct QueryWaitlistPositionsRequest {
    /// The list of the time off request ids for which to fetch the daily waitlist positions
    #[serde(rename = "timeOffRequests")]
    pub time_off_requests: Vec<crate::models::UserTimeOffRequestReference>,
}

impl QueryWaitlistPositionsRequest {
    pub fn new(time_off_requests: Vec<crate::models::UserTimeOffRequestReference>) -> QueryWaitlistPositionsRequest {
        QueryWaitlistPositionsRequest {
            time_off_requests,
        }
    }
}


