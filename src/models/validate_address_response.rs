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
pub struct ValidateAddressResponse {
    /// Was the passed in address valid
    #[serde(rename = "valid")]
    pub valid: bool,
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<crate::models::SubscriberResponse>>,
}

impl ValidateAddressResponse {
    pub fn new(valid: bool) -> ValidateAddressResponse {
        ValidateAddressResponse {
            valid,
            response: None,
        }
    }
}


