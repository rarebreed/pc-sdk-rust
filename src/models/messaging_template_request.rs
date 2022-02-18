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
pub struct MessagingTemplateRequest {
    /// A Response Management response identifier for a messaging template defined response
    #[serde(rename = "responseId", skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,
    /// A list of Response Management response substitutions for the response's messaging template
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::TemplateParameter>>,
}

impl MessagingTemplateRequest {
    pub fn new() -> MessagingTemplateRequest {
        MessagingTemplateRequest {
            response_id: None,
            parameters: None,
        }
    }
}


