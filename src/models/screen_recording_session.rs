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
pub struct ScreenRecordingSession {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
    /// The id of the communication that is being recorded on the conversation
    #[serde(rename = "communicationId", skip_serializing_if = "Option::is_none")]
    pub communication_id: Option<String>,
    #[serde(rename = "conversation", skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Box<crate::models::Conversation>>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl ScreenRecordingSession {
    pub fn new() -> ScreenRecordingSession {
        ScreenRecordingSession {
            id: None,
            name: None,
            user: None,
            communication_id: None,
            conversation: None,
            start_time: None,
            self_uri: None,
        }
    }
}


