/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ContentNotificationTemplate : Template notification object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentNotificationTemplate {
    /// The messaging provider template ID. For WhatsApp, 'namespace@name'.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Template language.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<crate::models::NotificationTemplateHeader>>,
    #[serde(rename = "body")]
    pub body: Box<crate::models::NotificationTemplateBody>,
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<Box<crate::models::NotificationTemplateFooter>>,
}

impl ContentNotificationTemplate {
    /// Template notification object.
    pub fn new(body: crate::models::NotificationTemplateBody) -> ContentNotificationTemplate {
        ContentNotificationTemplate {
            id: None,
            language: None,
            header: None,
            body: Box::new(body),
            footer: None,
        }
    }
}


