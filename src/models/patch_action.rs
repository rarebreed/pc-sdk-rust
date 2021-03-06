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
pub struct PatchAction {
    /// Media type of action.
    #[serde(rename = "mediaType")]
    pub media_type: MediaType,
    #[serde(rename = "actionTemplate", skip_serializing_if = "Option::is_none")]
    pub action_template: Option<Box<crate::models::ActionMapActionTemplate>>,
    #[serde(rename = "architectFlowFields", skip_serializing_if = "Option::is_none")]
    pub architect_flow_fields: Option<Box<crate::models::ArchitectFlowFields>>,
    #[serde(rename = "webMessagingOfferFields", skip_serializing_if = "Option::is_none")]
    pub web_messaging_offer_fields: Option<Box<crate::models::WebMessagingOfferFields>>,
    #[serde(rename = "openActionFields", skip_serializing_if = "Option::is_none")]
    pub open_action_fields: Option<Box<crate::models::OpenActionFields>>,
}

impl PatchAction {
    pub fn new(media_type: MediaType) -> PatchAction {
        PatchAction {
            media_type,
            action_template: None,
            architect_flow_fields: None,
            web_messaging_offer_fields: None,
            open_action_fields: None,
        }
    }
}

/// Media type of action.
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

