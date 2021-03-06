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
pub struct WfmHistoricalAdherenceQueryForUsers {
    /// Beginning of the date range to query in ISO-8601 format
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// End of the date range to query in ISO-8601 format. If it is not set, end date will be set to current time
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The time zone to use for returned results in olson format. If it is not set, the business unit time zone will be used to compute adherence
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// The userIds to report on
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
    /// Whether user exceptions should be returned as part of the results
    #[serde(rename = "includeExceptions", skip_serializing_if = "Option::is_none")]
    pub include_exceptions: Option<bool>,
}

impl WfmHistoricalAdherenceQueryForUsers {
    pub fn new(start_date: String, user_ids: Vec<String>) -> WfmHistoricalAdherenceQueryForUsers {
        WfmHistoricalAdherenceQueryForUsers {
            start_date,
            end_date: None,
            time_zone: None,
            user_ids,
            include_exceptions: None,
        }
    }
}


