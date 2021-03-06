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
pub struct EmployerInfo {
    #[serde(rename = "officialName", skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    #[serde(rename = "employeeId", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(rename = "employeeType", skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
    #[serde(rename = "dateHire", skip_serializing_if = "Option::is_none")]
    pub date_hire: Option<String>,
}

impl EmployerInfo {
    pub fn new() -> EmployerInfo {
        EmployerInfo {
            official_name: None,
            employee_id: None,
            employee_type: None,
            date_hire: None,
        }
    }
}


