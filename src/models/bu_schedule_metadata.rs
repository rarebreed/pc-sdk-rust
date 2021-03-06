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
pub struct BuScheduleMetadata {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The start week date for this schedule. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "weekDate", skip_serializing_if = "Option::is_none")]
    pub week_date: Option<String>,
    /// The number of weeks spanned by this schedule
    #[serde(rename = "weekCount", skip_serializing_if = "Option::is_none")]
    pub week_count: Option<i32>,
    /// The description of this schedule
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this schedule is published
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(rename = "shortTermForecast", skip_serializing_if = "Option::is_none")]
    pub short_term_forecast: Option<Box<crate::models::BuShortTermForecastReference>>,
    #[serde(rename = "generationResults", skip_serializing_if = "Option::is_none")]
    pub generation_results: Option<Box<crate::models::ScheduleGenerationResultSummary>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::WfmVersionedEntityMetadata>>,
    /// High level per-management unit schedule metadata
    #[serde(rename = "managementUnits", skip_serializing_if = "Option::is_none")]
    pub management_units: Option<Vec<crate::models::BuManagementUnitScheduleSummary>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl BuScheduleMetadata {
    pub fn new() -> BuScheduleMetadata {
        BuScheduleMetadata {
            id: None,
            week_date: None,
            week_count: None,
            description: None,
            published: None,
            short_term_forecast: None,
            generation_results: None,
            metadata: None,
            management_units: None,
            self_uri: None,
        }
    }
}


