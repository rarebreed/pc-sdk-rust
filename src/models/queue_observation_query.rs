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
pub struct QueueObservationQuery {
    #[serde(rename = "filter")]
    pub filter: Box<crate::models::QueueObservationQueryFilter>,
    /// Behaves like a SQL SELECT clause. Only named metrics will be retrieved.
    #[serde(rename = "metrics")]
    pub metrics: Vec<Metrics>,
    /// Metrics for which to include additional detailed observations
    #[serde(rename = "detailMetrics", skip_serializing_if = "Option::is_none")]
    pub detail_metrics: Option<Vec<DetailMetrics>>,
}

impl QueueObservationQuery {
    pub fn new(filter: crate::models::QueueObservationQueryFilter, metrics: Vec<Metrics>) -> QueueObservationQuery {
        QueueObservationQuery {
            filter: Box::new(filter),
            metrics,
            detail_metrics: None,
        }
    }
}

/// Behaves like a SQL SELECT clause. Only named metrics will be retrieved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metrics {
    #[serde(rename = "oActiveUsers")]
    OActiveUsers,
    #[serde(rename = "oAlerting")]
    OAlerting,
    #[serde(rename = "oInteracting")]
    OInteracting,
    #[serde(rename = "oMemberUsers")]
    OMemberUsers,
    #[serde(rename = "oOffQueueUsers")]
    OOffQueueUsers,
    #[serde(rename = "oOnQueueUsers")]
    OOnQueueUsers,
    #[serde(rename = "oUserPresences")]
    OUserPresences,
    #[serde(rename = "oUserRoutingStatuses")]
    OUserRoutingStatuses,
    #[serde(rename = "oWaiting")]
    OWaiting,
}

impl Default for Metrics {
    fn default() -> Metrics {
        Self::OActiveUsers
    }
}
/// Metrics for which to include additional detailed observations
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DetailMetrics {
    #[serde(rename = "oActiveUsers")]
    OActiveUsers,
    #[serde(rename = "oAlerting")]
    OAlerting,
    #[serde(rename = "oInteracting")]
    OInteracting,
    #[serde(rename = "oMemberUsers")]
    OMemberUsers,
    #[serde(rename = "oOffQueueUsers")]
    OOffQueueUsers,
    #[serde(rename = "oOnQueueUsers")]
    OOnQueueUsers,
    #[serde(rename = "oUserPresences")]
    OUserPresences,
    #[serde(rename = "oUserRoutingStatuses")]
    OUserRoutingStatuses,
    #[serde(rename = "oWaiting")]
    OWaiting,
}

impl Default for DetailMetrics {
    fn default() -> DetailMetrics {
        Self::OActiveUsers
    }
}

