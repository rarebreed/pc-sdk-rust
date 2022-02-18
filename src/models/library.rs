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
pub struct Library {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The library name.
    #[serde(rename = "name")]
    pub name: String,
    /// Current version for this resource.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::User>>,
    /// The date and time the response was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// This value is deprecated. Responses representing message templates may be added to any library.
    #[serde(rename = "responseType", skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Library {
    pub fn new(name: String) -> Library {
        Library {
            id: None,
            name,
            version: None,
            created_by: None,
            date_created: None,
            response_type: None,
            self_uri: None,
        }
    }
}

/// This value is deprecated. Responses representing message templates may be added to any library.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "MessagingTemplate")]
    MessagingTemplate,
    #[serde(rename = "CampaignSmsTemplate")]
    CampaignSmsTemplate,
    #[serde(rename = "CampaignEmailTemplate")]
    CampaignEmailTemplate,
}

impl Default for ResponseType {
    fn default() -> ResponseType {
        Self::MessagingTemplate
    }
}

