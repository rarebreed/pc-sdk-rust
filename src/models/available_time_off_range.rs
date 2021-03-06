/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// AvailableTimeOffRange : A list of available time off values in minutes and a number of time off requests currently in waitlist for each interval in requested date range, according to granularity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AvailableTimeOffRange {
    #[serde(rename = "timeOffLimit", skip_serializing_if = "Option::is_none")]
    pub time_off_limit: Option<Box<crate::models::TimeOffLimitReference>>,
    /// Start date of the requested date range. The end date is determined by the size of interval list. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Granularity choice for time off limit
    #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Granularity>,
    /// The list of available time off values in minutes per granularity interval
    #[serde(rename = "availableMinutesPerInterval", skip_serializing_if = "Option::is_none")]
    pub available_minutes_per_interval: Option<Vec<i32>>,
    /// The current number of waitlisted time off requests for every interval per granularity
    #[serde(rename = "waitlistedRequestsPerInterval", skip_serializing_if = "Option::is_none")]
    pub waitlisted_requests_per_interval: Option<Vec<i32>>,
    /// Whether the time off request can be waitlisted
    #[serde(rename = "waitlistEnabled", skip_serializing_if = "Option::is_none")]
    pub waitlist_enabled: Option<bool>,
}

impl AvailableTimeOffRange {
    /// A list of available time off values in minutes and a number of time off requests currently in waitlist for each interval in requested date range, according to granularity.
    pub fn new() -> AvailableTimeOffRange {
        AvailableTimeOffRange {
            time_off_limit: None,
            start_date: None,
            granularity: None,
            available_minutes_per_interval: None,
            waitlisted_requests_per_interval: None,
            waitlist_enabled: None,
        }
    }
}

/// Granularity choice for time off limit
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Granularity {
    #[serde(rename = "Daily")]
    Daily,
}

impl Default for Granularity {
    fn default() -> Granularity {
        Self::Daily
    }
}

