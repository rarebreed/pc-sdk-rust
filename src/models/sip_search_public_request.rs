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
pub struct SipSearchPublicRequest {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// unique identification of the placed call
    #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// SIP user to who the call was placed
    #[serde(rename = "toUser", skip_serializing_if = "Option::is_none")]
    pub to_user: Option<String>,
    /// SIP user who placed the call
    #[serde(rename = "fromUser", skip_serializing_if = "Option::is_none")]
    pub from_user: Option<String>,
    /// Unique identification of the conversation
    #[serde(rename = "conversationId", skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Unique identification of the participant
    #[serde(rename = "participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    /// Start date of the search. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStart")]
    pub date_start: String,
    /// End date of the search. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateEnd")]
    pub date_end: String,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl SipSearchPublicRequest {
    pub fn new(date_start: String, date_end: String) -> SipSearchPublicRequest {
        SipSearchPublicRequest {
            id: None,
            name: None,
            call_id: None,
            to_user: None,
            from_user: None,
            conversation_id: None,
            participant_id: None,
            date_start,
            date_end,
            self_uri: None,
        }
    }
}


