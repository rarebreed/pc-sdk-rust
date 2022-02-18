/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkPlanReference : Work plan information



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkPlanReference {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "managementUnit", skip_serializing_if = "Option::is_none")]
    pub management_unit: Option<Box<crate::models::ManagementUnitReference>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl WorkPlanReference {
    /// Work plan information
    pub fn new() -> WorkPlanReference {
        WorkPlanReference {
            id: None,
            management_unit: None,
            self_uri: None,
        }
    }
}


