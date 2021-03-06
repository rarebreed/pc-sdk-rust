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
pub struct LearningAssignmentAggregateQueryResponseMetric {
    /// The metric this applies to
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<Metric>,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<Box<crate::models::LearningAssignmentAggregateQueryResponseStats>>,
}

impl LearningAssignmentAggregateQueryResponseMetric {
    pub fn new() -> LearningAssignmentAggregateQueryResponseMetric {
        LearningAssignmentAggregateQueryResponseMetric {
            metric: None,
            stats: None,
        }
    }
}

/// The metric this applies to
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metric {
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
    #[serde(rename = "nPassedActivities")]
    NPassedActivities,
    #[serde(rename = "nFailedActivities")]
    NFailedActivities,
    #[serde(rename = "oActivityScore")]
    OActivityScore,
    #[serde(rename = "nNotCompletedActivities")]
    NNotCompletedActivities,
}

impl Default for Metric {
    fn default() -> Metric {
        Self::NActivities
    }
}

