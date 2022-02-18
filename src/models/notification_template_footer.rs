/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// NotificationTemplateFooter : Template footer object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationTemplateFooter {
    /// Footer text. For WhatsApp, ignored.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl NotificationTemplateFooter {
    /// Template footer object.
    pub fn new() -> NotificationTemplateFooter {
        NotificationTemplateFooter {
            text: None,
        }
    }
}


