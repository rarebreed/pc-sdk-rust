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
pub struct UpdateBusinessUnitRequest {
    /// The name of the business unit
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the division to which the business unit should be moved
    #[serde(rename = "divisionId", skip_serializing_if = "Option::is_none")]
    pub division_id: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::UpdateBusinessUnitSettings>>,
}

impl UpdateBusinessUnitRequest {
    pub fn new() -> UpdateBusinessUnitRequest {
        UpdateBusinessUnitRequest {
            name: None,
            division_id: None,
            settings: None,
        }
    }
}

