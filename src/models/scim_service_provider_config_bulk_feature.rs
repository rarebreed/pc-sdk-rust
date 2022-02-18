/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimServiceProviderConfigBulkFeature : Defines a \"bulk\" request in the SCIM service provider's configuration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimServiceProviderConfigBulkFeature {
    /// Indicates whether configuration options are supported.
    #[serde(rename = "supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<bool>,
    /// The maximum number of operations for each bulk request.
    #[serde(rename = "maxOperations", skip_serializing_if = "Option::is_none")]
    pub max_operations: Option<i32>,
    /// The maximum payload size.
    #[serde(rename = "maxPayloadSize", skip_serializing_if = "Option::is_none")]
    pub max_payload_size: Option<i32>,
}

impl ScimServiceProviderConfigBulkFeature {
    /// Defines a \"bulk\" request in the SCIM service provider's configuration.
    pub fn new() -> ScimServiceProviderConfigBulkFeature {
        ScimServiceProviderConfigBulkFeature {
            supported: None,
            max_operations: None,
            max_payload_size: None,
        }
    }
}

