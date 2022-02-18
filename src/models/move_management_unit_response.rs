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
pub struct MoveManagementUnitResponse {
    #[serde(rename = "businessUnit", skip_serializing_if = "Option::is_none")]
    pub business_unit: Option<Box<crate::models::BusinessUnitReference>>,
    /// The status of the move.  Will always be 'Processing' unless the Management Unit is already in the requested Business Unit in which case it will be 'Complete'
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl MoveManagementUnitResponse {
    pub fn new() -> MoveManagementUnitResponse {
        MoveManagementUnitResponse {
            business_unit: None,
            status: None,
        }
    }
}

/// The status of the move.  Will always be 'Processing' unless the Management Unit is already in the requested Business Unit in which case it will be 'Complete'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "Error")]
    Error,
}

impl Default for Status {
    fn default() -> Status {
        Self::Processing
    }
}

