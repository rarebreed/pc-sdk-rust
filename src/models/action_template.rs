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
pub struct ActionTemplate {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the action template.
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the action template's functionality.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Media type of action described by the action template.
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    /// Whether the action template is currently active, inactive or deleted.
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "contentOffer", skip_serializing_if = "Option::is_none")]
    pub content_offer: Option<Box<crate::models::ContentOffer>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// Date when action template was created in ISO-8601 format.
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// Date when action template was last modified in ISO-8601 format.
    #[serde(rename = "modifiedDate", skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<String>,
}

impl ActionTemplate {
    pub fn new(name: String, media_type: MediaType, state: State) -> ActionTemplate {
        ActionTemplate {
            id: None,
            name,
            description: None,
            media_type,
            state,
            content_offer: None,
            self_uri: None,
            created_date: None,
            modified_date: None,
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

