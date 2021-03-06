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
pub struct PatchBuReschedulingOptionsManagementUnitRequest {
    /// The management unit portion of the rescheduling run to update
    #[serde(rename = "managementUnitId")]
    pub management_unit_id: String,
    /// Whether to mark the run as applied.  Only applies to reschedule runs.  Once applied, a run cannot be un-marked as applied
    #[serde(rename = "applied", skip_serializing_if = "Option::is_none")]
    pub applied: Option<bool>,
}

impl PatchBuReschedulingOptionsManagementUnitRequest {
    pub fn new(management_unit_id: String) -> PatchBuReschedulingOptionsManagementUnitRequest {
        PatchBuReschedulingOptionsManagementUnitRequest {
            management_unit_id,
            applied: None,
        }
    }
}


