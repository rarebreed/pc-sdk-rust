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
pub struct ConversationBasic {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The external tag associated with the conversation.
    #[serde(rename = "externalTag", skip_serializing_if = "Option::is_none")]
    pub external_tag: Option<String>,
    /// The time when the conversation started. This will be the time when the first participant joined the conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// The time when the conversation ended. This will be the time when the last participant left the conversation, or null when the conversation is still active. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Identifiers of divisions associated with this conversation
    #[serde(rename = "divisions", skip_serializing_if = "Option::is_none")]
    pub divisions: Option<Vec<crate::models::ConversationDivisionMembership>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<crate::models::ParticipantBasic>>,
}

impl ConversationBasic {
    pub fn new(start_time: String) -> ConversationBasic {
        ConversationBasic {
            id: None,
            name: None,
            external_tag: None,
            start_time,
            end_time: None,
            divisions: None,
            self_uri: None,
            participants: None,
        }
    }
}


