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
pub struct WidgetClientConfigV1Http {
    #[serde(rename = "webChatSkin", skip_serializing_if = "Option::is_none")]
    pub web_chat_skin: Option<WebChatSkin>,
    #[serde(rename = "authenticationUrl", skip_serializing_if = "Option::is_none")]
    pub authentication_url: Option<String>,
}

impl WidgetClientConfigV1Http {
    pub fn new() -> WidgetClientConfigV1Http {
        WidgetClientConfigV1Http {
            web_chat_skin: None,
            authentication_url: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebChatSkin {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "modern-caret-skin")]
    ModernCaretSkin,
}

impl Default for WebChatSkin {
    fn default() -> WebChatSkin {
        Self::Basic
    }
}

