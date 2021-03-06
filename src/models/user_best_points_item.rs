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
pub struct UserBestPointsItem {
    /// Best points aggregation interval granularity
    #[serde(rename = "granularityType", skip_serializing_if = "Option::is_none")]
    pub granularity_type: Option<GranularityType>,
    /// Gamification points
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,
    /// Start workday of the best points aggregation interval. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateStartWorkday", skip_serializing_if = "Option::is_none")]
    pub date_start_workday: Option<String>,
    /// End workday of the best points aggregation interval. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateEndWorkday", skip_serializing_if = "Option::is_none")]
    pub date_end_workday: Option<String>,
    /// The rank of this user
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

impl UserBestPointsItem {
    pub fn new() -> UserBestPointsItem {
        UserBestPointsItem {
            granularity_type: None,
            points: None,
            date_start_workday: None,
            date_end_workday: None,
            rank: None,
        }
    }
}

/// Best points aggregation interval granularity
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GranularityType {
    #[serde(rename = "Monthly")]
    Monthly,
    #[serde(rename = "Weekly")]
    Weekly,
    #[serde(rename = "Daily")]
    Daily,
}

impl Default for GranularityType {
    fn default() -> GranularityType {
        Self::Monthly
    }
}

