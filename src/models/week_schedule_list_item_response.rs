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
pub struct WeekScheduleListItemResponse {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// First day of this week schedule in yyyy-MM-dd format
    #[serde(rename = "weekDate", skip_serializing_if = "Option::is_none")]
    pub week_date: Option<String>,
    /// Description of the week schedule
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the week schedule is published
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(rename = "generationResults", skip_serializing_if = "Option::is_none")]
    pub generation_results: Option<Box<crate::models::WeekScheduleGenerationResult>>,
    #[serde(rename = "shortTermForecast", skip_serializing_if = "Option::is_none")]
    pub short_term_forecast: Option<Box<crate::models::ShortTermForecastReference>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::WfmVersionedEntityMetadata>>,
}

impl WeekScheduleListItemResponse {
    pub fn new() -> WeekScheduleListItemResponse {
        WeekScheduleListItemResponse {
            id: None,
            self_uri: None,
            week_date: None,
            description: None,
            published: None,
            generation_results: None,
            short_term_forecast: None,
            metadata: None,
        }
    }
}

