/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimServiceProviderConfigFilterFeature : Defines a \"filter\" request in the SCIM service provider's configuration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimServiceProviderConfigFilterFeature {
    /// Indicates whether configuration options are supported.
    #[serde(rename = "supported", skip_serializing_if = "Option::is_none")]
    pub supported: Option<bool>,
    /// The maximum number of results returned from a filtered query.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ScimServiceProviderConfigFilterFeature {
    /// Defines a \"filter\" request in the SCIM service provider's configuration.
    pub fn new() -> ScimServiceProviderConfigFilterFeature {
        ScimServiceProviderConfigFilterFeature {
            supported: None,
            max_results: None,
        }
    }
}

