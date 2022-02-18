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
pub struct Education {
    #[serde(rename = "school", skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    #[serde(rename = "fieldOfStudy", skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<String>,
    /// Notes about education has a 2000 character limit
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateStart", skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,
    /// Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd
    #[serde(rename = "dateEnd", skip_serializing_if = "Option::is_none")]
    pub date_end: Option<String>,
}

impl Education {
    pub fn new() -> Education {
        Education {
            school: None,
            field_of_study: None,
            notes: None,
            date_start: None,
            date_end: None,
        }
    }
}


