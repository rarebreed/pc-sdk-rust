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
pub struct InboundMessageRequest {
    /// The ID of the queue to use for routing the email conversation. This field is mutually exclusive with flowId
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    /// The ID of the flow to use for routing email conversation. This field is mutually exclusive with queueId
    #[serde(rename = "flowId", skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// The name of the provider that is sourcing the email such as Oracle, Salesforce, etc.
    #[serde(rename = "provider")]
    pub provider: String,
    /// The list of skill ID's to use for routing.
    #[serde(rename = "skillIds", skip_serializing_if = "Option::is_none")]
    pub skill_ids: Option<Vec<String>>,
    /// The ID of the language to use for routing.
    #[serde(rename = "languageId", skip_serializing_if = "Option::is_none")]
    pub language_id: Option<String>,
    /// The priority to assign to the conversation for routing.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// The list of attributes to associate with the customer participant.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// The email address of the recipient of the email.
    #[serde(rename = "toAddress", skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    /// The name of the recipient of the email.
    #[serde(rename = "toName", skip_serializing_if = "Option::is_none")]
    pub to_name: Option<String>,
    /// The email address of the sender of the email.
    #[serde(rename = "fromAddress", skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// The name of the sender of the email.
    #[serde(rename = "fromName", skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// The subject of the email
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl InboundMessageRequest {
    pub fn new(provider: String) -> InboundMessageRequest {
        InboundMessageRequest {
            queue_id: None,
            flow_id: None,
            provider,
            skill_ids: None,
            language_id: None,
            priority: None,
            attributes: None,
            to_address: None,
            to_name: None,
            from_address: None,
            from_name: None,
            subject: None,
        }
    }
}


