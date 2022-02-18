/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ConversationContentGeneric : Generic content object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationContentGeneric {
    /// Text to show in the title.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Text to show in the description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of an image.
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// URL of a video.
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<String>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Box<crate::models::ConversationContentActions>>,
    /// An array of component objects.
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<crate::models::ConversationButtonComponent>>,
}

impl ConversationContentGeneric {
    /// Generic content object.
    pub fn new() -> ConversationContentGeneric {
        ConversationContentGeneric {
            title: None,
            description: None,
            image: None,
            video: None,
            actions: None,
            components: None,
        }
    }
}

