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
pub struct AutomaticTimeZoneMappingSettings {
    /// The time intervals to use for automatic time zone mapping.
    #[serde(rename = "callableWindows", skip_serializing_if = "Option::is_none")]
    pub callable_windows: Option<Vec<crate::models::CallableWindow>>,
}

impl AutomaticTimeZoneMappingSettings {
    pub fn new() -> AutomaticTimeZoneMappingSettings {
        AutomaticTimeZoneMappingSettings {
            callable_windows: None,
        }
    }
}


