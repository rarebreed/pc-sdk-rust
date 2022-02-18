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
pub struct ErrorBody {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "entityName", skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(rename = "messageWithParams", skip_serializing_if = "Option::is_none")]
    pub message_with_params: Option<String>,
    #[serde(rename = "messageParams", skip_serializing_if = "Option::is_none")]
    pub message_params: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::Detail>>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::ErrorBody>>,
}

impl ErrorBody {
    pub fn new() -> ErrorBody {
        ErrorBody {
            message: None,
            code: None,
            status: None,
            entity_id: None,
            entity_name: None,
            message_with_params: None,
            message_params: None,
            context_id: None,
            details: None,
            errors: None,
        }
    }
}

