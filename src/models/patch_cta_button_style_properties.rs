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
pub struct PatchCtaButtonStyleProperties {
    /// Color of the text. (eg. #FFFFFF)
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Font of the text. (eg. Helvetica)
    #[serde(rename = "font", skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// Font size of the text. (eg. '12')
    #[serde(rename = "fontSize", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    /// Text alignment.
    #[serde(rename = "textAlign", skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextAlign>,
    /// Background color of the CTA button. (eg. #A04033)
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
}

impl PatchCtaButtonStyleProperties {
    pub fn new() -> PatchCtaButtonStyleProperties {
        PatchCtaButtonStyleProperties {
            color: None,
            font: None,
            font_size: None,
            text_align: None,
            background_color: None,
        }
    }
}

/// Text alignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TextAlign {
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "Right")]
    Right,
    #[serde(rename = "Center")]
    Center,
}

impl Default for TextAlign {
    fn default() -> TextAlign {
        Self::Left
    }
}

