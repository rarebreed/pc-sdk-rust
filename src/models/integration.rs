/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// Integration : Details for an Integration



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Integration {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the integration, used to distinguish this integration from others of the same type.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "integrationType", skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<Box<crate::models::IntegrationType>>,
    /// Notes about the integration.
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Configured state of the integration.
    #[serde(rename = "intendedState")]
    pub intended_state: IntendedState,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::IntegrationConfigurationInfo>>,
    #[serde(rename = "reportedState", skip_serializing_if = "Option::is_none")]
    pub reported_state: Option<Box<crate::models::IntegrationStatusInfo>>,
    /// Read-only attributes for the integration.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Integration {
    /// Details for an Integration
    pub fn new(intended_state: IntendedState) -> Integration {
        Integration {
            id: None,
            name: None,
            integration_type: None,
            notes: None,
            intended_state,
            config: None,
            reported_state: None,
            attributes: None,
            self_uri: None,
        }
    }
}

/// Configured state of the integration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IntendedState {
    #[serde(rename = "ENABLED")]
    ENABLED,
    #[serde(rename = "DISABLED")]
    DISABLED,
    #[serde(rename = "DELETED")]
    DELETED,
}

impl Default for IntendedState {
    fn default() -> IntendedState {
        Self::ENABLED
    }
}

