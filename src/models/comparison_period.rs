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
pub struct ComparisonPeriod {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Key Performance Indicator optimised during the comparison period.
    #[serde(rename = "kpi", skip_serializing_if = "Option::is_none")]
    pub kpi: Option<String>,
    /// Start date of the comparison period. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStarted", skip_serializing_if = "Option::is_none")]
    pub date_started: Option<String>,
    /// End date of the comparison period. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateEnded", skip_serializing_if = "Option::is_none")]
    pub date_ended: Option<String>,
    /// Absolute metric (in which the KPI is based) total for the interactions handled by predictive routing (GPR was on)
    #[serde(rename = "kpiTotalOn", skip_serializing_if = "Option::is_none")]
    pub kpi_total_on: Option<i64>,
    /// Absolute metric (in which the KPI is based) total for the interactions not routed by predictive routing (GPR was off)
    #[serde(rename = "kpiTotalOff", skip_serializing_if = "Option::is_none")]
    pub kpi_total_off: Option<i64>,
    /// Total interactions handled by predictive routing (GPR was on)
    #[serde(rename = "interactionCountOn", skip_serializing_if = "Option::is_none")]
    pub interaction_count_on: Option<i64>,
    /// Total interactions not routed by predictive routing (GPR was off)
    #[serde(rename = "interactionCountOff", skip_serializing_if = "Option::is_none")]
    pub interaction_count_off: Option<i64>,
    /// KPI results for each metric
    #[serde(rename = "kpiResults", skip_serializing_if = "Option::is_none")]
    pub kpi_results: Option<Vec<crate::models::KpiResult>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl ComparisonPeriod {
    pub fn new() -> ComparisonPeriod {
        ComparisonPeriod {
            id: None,
            kpi: None,
            date_started: None,
            date_ended: None,
            kpi_total_on: None,
            kpi_total_off: None,
            interaction_count_on: None,
            interaction_count_off: None,
            kpi_results: None,
            self_uri: None,
        }
    }
}


