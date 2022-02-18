/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// DraftValidationResult : Validation results



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DraftValidationResult {
    /// Indicates if configuration is valid
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    /// List of errors causing validation failure
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::ErrorBody>>,
}

impl DraftValidationResult {
    /// Validation results
    pub fn new() -> DraftValidationResult {
        DraftValidationResult {
            valid: None,
            errors: None,
        }
    }
}

