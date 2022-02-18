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
pub struct Metrics {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The order of metric within a performance profile
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// The name of associated metric definition
    #[serde(rename = "metricDefinitionName", skip_serializing_if = "Option::is_none")]
    pub metric_definition_name: Option<String>,
    /// The id of associated metric definition
    #[serde(rename = "metricDefinitionId", skip_serializing_if = "Option::is_none")]
    pub metric_definition_id: Option<String>,
    /// The id of associated external metric definition
    #[serde(rename = "externalMetricDefinitionId", skip_serializing_if = "Option::is_none")]
    pub external_metric_definition_id: Option<String>,
    /// Corresponding unit type for this metric
    #[serde(rename = "unitType", skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<UnitType>,
    /// A flag for whether this metric is enabled for a performance profile
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The name of associated objective template
    #[serde(rename = "templateName", skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// Achievable maximum points for this metric
    #[serde(rename = "maxPoints", skip_serializing_if = "Option::is_none")]
    pub max_points: Option<i32>,
    /// Performance profile id of this metric
    #[serde(rename = "performanceProfileId", skip_serializing_if = "Option::is_none")]
    pub performance_profile_id: Option<String>,
    #[serde(rename = "linkedMetric", skip_serializing_if = "Option::is_none")]
    pub linked_metric: Option<Box<crate::models::AddressableEntityRef>>,
    /// The created date of this metric. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The unlinked workday for this metric if this metric was ever unlinked. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateUnlinked", skip_serializing_if = "Option::is_none")]
    pub date_unlinked: Option<String>,
    #[serde(rename = "sourcePerformanceProfile", skip_serializing_if = "Option::is_none")]
    pub source_performance_profile: Option<Box<crate::models::PerformanceProfile>>,
    /// Unit definition of linked external metric
    #[serde(rename = "unitDefinition", skip_serializing_if = "Option::is_none")]
    pub unit_definition: Option<String>,
    /// Precision of linked external metric
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            id: None,
            name: None,
            order: None,
            metric_definition_name: None,
            metric_definition_id: None,
            external_metric_definition_id: None,
            unit_type: None,
            enabled: None,
            template_name: None,
            max_points: None,
            performance_profile_id: None,
            linked_metric: None,
            date_created: None,
            date_unlinked: None,
            source_performance_profile: None,
            unit_definition: None,
            precision: None,
            self_uri: None,
        }
    }
}

/// Corresponding unit type for this metric
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnitType {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Percent")]
    Percent,
    #[serde(rename = "Currency")]
    Currency,
    #[serde(rename = "Seconds")]
    Seconds,
    #[serde(rename = "Number")]
    Number,
    #[serde(rename = "AttendanceStatus")]
    AttendanceStatus,
    #[serde(rename = "Unit")]
    Unit,
}

impl Default for UnitType {
    fn default() -> UnitType {
        Self::None
    }
}

