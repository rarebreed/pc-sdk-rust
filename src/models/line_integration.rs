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
pub struct LineIntegration {
    /// A unique Integration Id
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the LINE Integration
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "supportedContent", skip_serializing_if = "Option::is_none")]
    pub supported_content: Option<Box<crate::models::SupportedContentReference>>,
    /// The Channel Id from LINE messenger
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// The Webhook URI to be updated in LINE platform
    #[serde(rename = "webhookUri")]
    pub webhook_uri: String,
    /// The status of the LINE Integration
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Box<crate::models::DomainEntityRef>>,
    /// Date this Integration was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date this Integration was modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::DomainEntityRef>>,
    /// Version number required for updates.
    #[serde(rename = "version")]
    pub version: i32,
    /// Status of asynchronous create operation
    #[serde(rename = "createStatus", skip_serializing_if = "Option::is_none")]
    pub create_status: Option<CreateStatus>,
    #[serde(rename = "createError", skip_serializing_if = "Option::is_none")]
    pub create_error: Option<Box<crate::models::ErrorBody>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl LineIntegration {
    pub fn new(id: String, name: String, channel_id: String, webhook_uri: String, version: i32) -> LineIntegration {
        LineIntegration {
            id,
            name,
            supported_content: None,
            channel_id,
            webhook_uri,
            status: None,
            recipient: None,
            date_created: None,
            date_modified: None,
            created_by: None,
            modified_by: None,
            version,
            create_status: None,
            create_error: None,
            self_uri: None,
        }
    }
}

/// Status of asynchronous create operation
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateStatus {
    #[serde(rename = "Initiated")]
    Initiated,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Error")]
    Error,
}

impl Default for CreateStatus {
    fn default() -> CreateStatus {
        Self::Initiated
    }
}

