/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ButtonComponent : Structured template button object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ButtonComponent {
    /// Text to show inside the button.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Box<crate::models::ContentActions>>,
}

impl ButtonComponent {
    /// Structured template button object.
    pub fn new() -> ButtonComponent {
        ButtonComponent {
            title: None,
            actions: None,
        }
    }
}

