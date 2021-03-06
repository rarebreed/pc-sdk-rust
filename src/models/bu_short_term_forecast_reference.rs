/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// BuShortTermForecastReference : A pointer to a short term forecast



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BuShortTermForecastReference {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The weekDate of the short term forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "weekDate")]
    pub week_date: String,
    /// The description of the short term forecast
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl BuShortTermForecastReference {
    /// A pointer to a short term forecast
    pub fn new(week_date: String) -> BuShortTermForecastReference {
        BuShortTermForecastReference {
            id: None,
            week_date,
            description: None,
            self_uri: None,
        }
    }
}


