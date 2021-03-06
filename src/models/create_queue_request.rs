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
pub struct CreateQueueRequest {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The queue name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::WritableDivision>>,
    /// The queue description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The date the queue was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date of the last modification to the queue. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// The ID of the user that last modified the queue.
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// The ID of the user that created the queue.
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The total number of members in the queue.
    #[serde(rename = "memberCount", skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// The number of user members (i.e., non-group members) in the queue.
    #[serde(rename = "userMemberCount", skip_serializing_if = "Option::is_none")]
    pub user_member_count: Option<i32>,
    /// The number of joined members in the queue.
    #[serde(rename = "joinedMemberCount", skip_serializing_if = "Option::is_none")]
    pub joined_member_count: Option<i32>,
    /// The media settings for the queue. Valid key values: CALL, CALLBACK, CHAT, EMAIL, MESSAGE, SOCIAL_EXPRESSION, VIDEO_COMM
    #[serde(rename = "mediaSettings", skip_serializing_if = "Option::is_none")]
    pub media_settings: Option<::std::collections::HashMap<String, crate::models::MediaSetting>>,
    /// The routing rules for the queue, used for routing to known or preferred agents.
    #[serde(rename = "routingRules", skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<Vec<crate::models::RoutingRule>>,
    #[serde(rename = "bullseye", skip_serializing_if = "Option::is_none")]
    pub bullseye: Option<Box<crate::models::Bullseye>>,
    #[serde(rename = "acwSettings", skip_serializing_if = "Option::is_none")]
    pub acw_settings: Option<Box<crate::models::AcwSettings>>,
    /// The skill evaluation method to use when routing conversations.
    #[serde(rename = "skillEvaluationMethod", skip_serializing_if = "Option::is_none")]
    pub skill_evaluation_method: Option<SkillEvaluationMethod>,
    #[serde(rename = "queueFlow", skip_serializing_if = "Option::is_none")]
    pub queue_flow: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "emailInQueueFlow", skip_serializing_if = "Option::is_none")]
    pub email_in_queue_flow: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "messageInQueueFlow", skip_serializing_if = "Option::is_none")]
    pub message_in_queue_flow: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "whisperPrompt", skip_serializing_if = "Option::is_none")]
    pub whisper_prompt: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "onHoldPrompt", skip_serializing_if = "Option::is_none")]
    pub on_hold_prompt: Option<Box<crate::models::DomainEntityRef>>,
    /// Specifies whether the configured whisper should play for all ACD calls, or only for those which are auto-answered.
    #[serde(rename = "autoAnswerOnly", skip_serializing_if = "Option::is_none")]
    pub auto_answer_only: Option<bool>,
    /// Indicates whether voice transcription is enabled for this queue.
    #[serde(rename = "enableTranscription", skip_serializing_if = "Option::is_none")]
    pub enable_transcription: Option<bool>,
    /// Indicates whether manual assignment is enabled for this queue.
    #[serde(rename = "enableManualAssignment", skip_serializing_if = "Option::is_none")]
    pub enable_manual_assignment: Option<bool>,
    /// The name to use for caller identification for outbound calls from this queue.
    #[serde(rename = "callingPartyName", skip_serializing_if = "Option::is_none")]
    pub calling_party_name: Option<String>,
    /// The phone number to use for caller identification for outbound calls from this queue.
    #[serde(rename = "callingPartyNumber", skip_serializing_if = "Option::is_none")]
    pub calling_party_number: Option<String>,
    /// The default script Ids for the communication types.
    #[serde(rename = "defaultScripts", skip_serializing_if = "Option::is_none")]
    pub default_scripts: Option<::std::collections::HashMap<String, crate::models::Script>>,
    #[serde(rename = "outboundMessagingAddresses", skip_serializing_if = "Option::is_none")]
    pub outbound_messaging_addresses: Option<Box<crate::models::QueueMessagingAddresses>>,
    #[serde(rename = "outboundEmailAddress", skip_serializing_if = "Option::is_none")]
    pub outbound_email_address: Option<Box<crate::models::QueueEmailAddress>>,
    /// The id of an existing queue to copy the settings from when creating a new queue.
    #[serde(rename = "sourceQueueId", skip_serializing_if = "Option::is_none")]
    pub source_queue_id: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl CreateQueueRequest {
    pub fn new(name: String) -> CreateQueueRequest {
        CreateQueueRequest {
            id: None,
            name,
            division: None,
            description: None,
            date_created: None,
            date_modified: None,
            modified_by: None,
            created_by: None,
            member_count: None,
            user_member_count: None,
            joined_member_count: None,
            media_settings: None,
            routing_rules: None,
            bullseye: None,
            acw_settings: None,
            skill_evaluation_method: None,
            queue_flow: None,
            email_in_queue_flow: None,
            message_in_queue_flow: None,
            whisper_prompt: None,
            on_hold_prompt: None,
            auto_answer_only: None,
            enable_transcription: None,
            enable_manual_assignment: None,
            calling_party_name: None,
            calling_party_number: None,
            default_scripts: None,
            outbound_messaging_addresses: None,
            outbound_email_address: None,
            source_queue_id: None,
            self_uri: None,
        }
    }
}

/// The skill evaluation method to use when routing conversations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SkillEvaluationMethod {
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "BEST")]
    BEST,
    #[serde(rename = "ALL")]
    ALL,
}

impl Default for SkillEvaluationMethod {
    fn default() -> SkillEvaluationMethod {
        Self::NONE
    }
}

