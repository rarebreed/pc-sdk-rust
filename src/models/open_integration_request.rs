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
pub struct OpenIntegrationRequest {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the Open messaging integration.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "supportedContent", skip_serializing_if = "Option::is_none")]
    pub supported_content: Option<Box<crate::models::SupportedContentReference>>,
    /// The outbound notification webhook URL for the Open messaging integration.
    #[serde(rename = "outboundNotificationWebhookUrl")]
    pub outbound_notification_webhook_url: String,
    /// The outbound notification webhook signature secret token.
    #[serde(rename = "outboundNotificationWebhookSignatureSecretToken")]
    pub outbound_notification_webhook_signature_secret_token: String,
    /// The user specified headers for the Open messaging integration.
    #[serde(rename = "webhookHeaders", skip_serializing_if = "Option::is_none")]
    pub webhook_headers: Option<::std::collections::HashMap<String, String>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl OpenIntegrationRequest {
    pub fn new(name: String, outbound_notification_webhook_url: String, outbound_notification_webhook_signature_secret_token: String) -> OpenIntegrationRequest {
        OpenIntegrationRequest {
            id: None,
            name,
            supported_content: None,
            outbound_notification_webhook_url,
            outbound_notification_webhook_signature_secret_token,
            webhook_headers: None,
            self_uri: None,
        }
    }
}


