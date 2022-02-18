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
pub struct BuShortTermForecast {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The start week date of this forecast in yyyy-MM-dd.  Must fall on the start day of week for the associated business unit. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "weekDate", skip_serializing_if = "Option::is_none")]
    pub week_date: Option<String>,
    /// The number of weeks this forecast covers
    #[serde(rename = "weekCount", skip_serializing_if = "Option::is_none")]
    pub week_count: Option<i32>,
    /// The method by which this forecast was created
    #[serde(rename = "creationMethod", skip_serializing_if = "Option::is_none")]
    pub creation_method: Option<CreationMethod>,
    /// The description of this forecast
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this forecast contains modifications on legacy metrics
    #[serde(rename = "legacy", skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::WfmVersionedEntityMetadata>>,
    /// Whether this forecast can be used for scheduling
    #[serde(rename = "canUseForScheduling", skip_serializing_if = "Option::is_none")]
    pub can_use_for_scheduling: Option<bool>,
    /// The reference start date for interval-based data for this forecast. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "referenceStartDate", skip_serializing_if = "Option::is_none")]
    pub reference_start_date: Option<String>,
    /// The source day pointers for this forecast
    #[serde(rename = "sourceDays", skip_serializing_if = "Option::is_none")]
    pub source_days: Option<Vec<crate::models::ForecastSourceDayPointer>>,
    /// Any manual modifications applied to this forecast
    #[serde(rename = "modifications", skip_serializing_if = "Option::is_none")]
    pub modifications: Option<Vec<crate::models::BuForecastModification>>,
    #[serde(rename = "generationResults", skip_serializing_if = "Option::is_none")]
    pub generation_results: Option<Box<crate::models::BuForecastGenerationResult>>,
    /// The time zone for this forecast
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// The version of the planning groups that was used for this forecast
    #[serde(rename = "planningGroupsVersion", skip_serializing_if = "Option::is_none")]
    pub planning_groups_version: Option<i32>,
    #[serde(rename = "planningGroups", skip_serializing_if = "Option::is_none")]
    pub planning_groups: Option<Box<crate::models::ForecastPlanningGroupsResponse>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl BuShortTermForecast {
    pub fn new() -> BuShortTermForecast {
        BuShortTermForecast {
            id: None,
            week_date: None,
            week_count: None,
            creation_method: None,
            description: None,
            legacy: None,
            metadata: None,
            can_use_for_scheduling: None,
            reference_start_date: None,
            source_days: None,
            modifications: None,
            generation_results: None,
            time_zone: None,
            planning_groups_version: None,
            planning_groups: None,
            self_uri: None,
        }
    }
}

/// The method by which this forecast was created
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreationMethod {
    #[serde(rename = "Import")]
    Import,
    #[serde(rename = "ImportedHistoricalWeightedAverage")]
    ImportedHistoricalWeightedAverage,
    #[serde(rename = "HistoricalWeightedAverage")]
    HistoricalWeightedAverage,
    #[serde(rename = "Advanced")]
    Advanced,
}

impl Default for CreationMethod {
    fn default() -> CreationMethod {
        Self::Import
    }
}

