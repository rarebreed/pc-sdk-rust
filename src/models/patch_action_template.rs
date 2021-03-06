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
pub struct PatchActionTemplate {
    /// Name of the action template.
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the action template's functionality.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Media type of action described by the action template.
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    /// Whether the action template is currently active, inactive or deleted.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "contentOffer", skip_serializing_if = "Option::is_none")]
    pub content_offer: Option<Box<crate::models::PatchContentOffer>>,
}

impl PatchActionTemplate {
    pub fn new(name: String) -> PatchActionTemplate {
        PatchActionTemplate {
            name,
            description: None,
            media_type: None,
            state: None,
            content_offer: None,
        }
    }
}

/// Media type of action described by the action template.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "webchat")]
    Webchat,
    #[serde(rename = "webMessagingOffer")]
    WebMessagingOffer,
    #[serde(rename = "contentOffer")]
    ContentOffer,
    #[serde(rename = "integrationAction")]
    IntegrationAction,
    #[serde(rename = "architectFlow")]
    ArchitectFlow,
    #[serde(rename = "openAction")]
    OpenAction,
}

impl Default for MediaType {
    fn default() -> MediaType {
        Self::Webchat
    }
}
/// Whether the action template is currently active, inactive or deleted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "Deleted")]
    Deleted,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}

