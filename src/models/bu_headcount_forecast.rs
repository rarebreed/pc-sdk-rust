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
pub struct BuHeadcountForecast {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::BuPlanningGroupHeadcountForecast>>,
    /// Reference start date for the interval values in each forecast entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "referenceStartDate", skip_serializing_if = "Option::is_none")]
    pub reference_start_date: Option<String>,
}

impl BuHeadcountForecast {
    pub fn new() -> BuHeadcountForecast {
        BuHeadcountForecast {
            entities: None,
            reference_start_date: None,
        }
    }
}


