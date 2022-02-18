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
pub struct SingleWorkdayAveragePoints {
    /// Queried target workday. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateWorkday", skip_serializing_if = "Option::is_none")]
    pub date_workday: Option<String>,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::Division>>,
    /// The average points per agent earned within the division
    #[serde(rename = "averagePoints", skip_serializing_if = "Option::is_none")]
    pub average_points: Option<f64>,
    #[serde(rename = "performanceProfile", skip_serializing_if = "Option::is_none")]
    pub performance_profile: Option<Box<crate::models::AddressableEntityRef>>,
}

impl SingleWorkdayAveragePoints {
    pub fn new() -> SingleWorkdayAveragePoints {
        SingleWorkdayAveragePoints {
            date_workday: None,
            division: None,
            average_points: None,
            performance_profile: None,
        }
    }
}

