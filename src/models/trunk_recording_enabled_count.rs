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
pub struct TrunkRecordingEnabledCount {
    /// The amount of trunks that have recording enabled
    #[serde(rename = "enabledCount", skip_serializing_if = "Option::is_none")]
    pub enabled_count: Option<i32>,
    /// The amount of trunks that do not have recording enabled
    #[serde(rename = "disabledCount", skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
}

impl TrunkRecordingEnabledCount {
    pub fn new() -> TrunkRecordingEnabledCount {
        TrunkRecordingEnabledCount {
            enabled_count: None,
            disabled_count: None,
        }
    }
}


