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
pub struct PatchCloseButtonStyleProperties {
    /// Color of button. (eg. #FF0000)
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Opacity of button.
    #[serde(rename = "opacity", skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,
}

impl PatchCloseButtonStyleProperties {
    pub fn new() -> PatchCloseButtonStyleProperties {
        PatchCloseButtonStyleProperties {
            color: None,
            opacity: None,
        }
    }
}

