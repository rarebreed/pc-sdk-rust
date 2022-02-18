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
pub struct CoachingAppointmentAggregateRequest {
    /// Interval to aggregate across. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss
    #[serde(rename = "interval")]
    pub interval: String,
    /// A list of metrics to aggregate.  If omitted, all metrics are returned.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<Metrics>>,
    /// An optional list of items by which to group the result data.
    #[serde(rename = "groupBy", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupBy>>,
    #[serde(rename = "filter")]
    pub filter: Box<crate::models::QueryRequestFilter>,
}

impl CoachingAppointmentAggregateRequest {
    pub fn new(interval: String, filter: crate::models::QueryRequestFilter) -> CoachingAppointmentAggregateRequest {
        CoachingAppointmentAggregateRequest {
            interval,
            metrics: None,
            group_by: None,
            filter: Box::new(filter),
        }
    }
}

/// A list of metrics to aggregate.  If omitted, all metrics are returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metrics {
    #[serde(rename = "nActivities")]
    NActivities,
    #[serde(rename = "nPlannedActivities")]
    NPlannedActivities,
    #[serde(rename = "nInProgressActivities")]
    NInProgressActivities,
    #[serde(rename = "nCompleteActivities")]
    NCompleteActivities,
    #[serde(rename = "nOverdueActivities")]
    NOverdueActivities,
    #[serde(rename = "nInvalidScheduleActivities")]
    NInvalidScheduleActivities,
}

impl Default for Metrics {
    fn default() -> Metrics {
        Self::NActivities
    }
}
/// An optional list of items by which to group the result data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupBy {
    #[serde(rename = "attendeeId")]
    AttendeeId,
}

impl Default for GroupBy {
    fn default() -> GroupBy {
        Self::AttendeeId
    }
}
