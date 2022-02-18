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
pub struct CopyWorkPlanRotationRequest {
    /// Name to apply to the new copy of the work plan rotation
    #[serde(rename = "name")]
    pub name: String,
}

impl CopyWorkPlanRotationRequest {
    pub fn new(name: String) -> CopyWorkPlanRotationRequest {
        CopyWorkPlanRotationRequest {
            name,
        }
    }
}


