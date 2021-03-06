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
pub struct PatchCallToAction {
    /// Text displayed on the call to action button.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// URL to open when user clicks on the call to action button.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Where the URL should be opened when the user clicks on the call to action button.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}

impl PatchCallToAction {
    pub fn new() -> PatchCallToAction {
        PatchCallToAction {
            text: None,
            url: None,
            target: None,
        }
    }
}

/// Where the URL should be opened when the user clicks on the call to action button.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Target {
    #[serde(rename = "Blank")]
    Blank,
    #[serde(rename = "Self")]
    _Self,
}

impl Default for Target {
    fn default() -> Target {
        Self::Blank
    }
}

