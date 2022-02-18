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
pub struct BuRescheduleAgentScheduleResult {
    #[serde(rename = "managementUnit", skip_serializing_if = "Option::is_none")]
    pub management_unit: Option<Box<crate::models::ManagementUnitReference>>,
    #[serde(rename = "downloadResult", skip_serializing_if = "Option::is_none")]
    pub download_result: Option<Box<crate::models::MuRescheduleResultWrapper>>,
    /// The download URL from which to fetch the result
    #[serde(rename = "downloadUrl", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

impl BuRescheduleAgentScheduleResult {
    pub fn new() -> BuRescheduleAgentScheduleResult {
        BuRescheduleAgentScheduleResult {
            management_unit: None,
            download_result: None,
            download_url: None,
        }
    }
}

