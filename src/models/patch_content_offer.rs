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
pub struct PatchContentOffer {
    /// URL for image displayed to the customer when displaying content offer.
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The display mode of Genesys Widgets when displaying content offer.
    #[serde(rename = "displayMode", skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<DisplayMode>,
    /// The layout mode of the text shown to the user when displaying content offer.
    #[serde(rename = "layoutMode", skip_serializing_if = "Option::is_none")]
    pub layout_mode: Option<LayoutMode>,
    /// Title used in the header of the content offer.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Headline displayed above the body text of the content offer.
    #[serde(rename = "headline", skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// Body text of the content offer.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "callToAction", skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<Box<crate::models::PatchCallToAction>>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Box<crate::models::PatchContentOfferStylingConfiguration>>,
}

impl PatchContentOffer {
    pub fn new() -> PatchContentOffer {
        PatchContentOffer {
            image_url: None,
            display_mode: None,
            layout_mode: None,
            title: None,
            headline: None,
            body: None,
            call_to_action: None,
            style: None,
        }
    }
}

/// The display mode of Genesys Widgets when displaying content offer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisplayMode {
    #[serde(rename = "Modal")]
    Modal,
    #[serde(rename = "Overlay")]
    Overlay,
    #[serde(rename = "Toast")]
    Toast,
}

impl Default for DisplayMode {
    fn default() -> DisplayMode {
        Self::Modal
    }
}
/// The layout mode of the text shown to the user when displaying content offer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LayoutMode {
    #[serde(rename = "TextOnly")]
    TextOnly,
    #[serde(rename = "ImageOnly")]
    ImageOnly,
    #[serde(rename = "LeftText")]
    LeftText,
    #[serde(rename = "RightText")]
    RightText,
    #[serde(rename = "TopText")]
    TopText,
    #[serde(rename = "BottomText")]
    BottomText,
}

impl Default for LayoutMode {
    fn default() -> LayoutMode {
        Self::TextOnly
    }
}

