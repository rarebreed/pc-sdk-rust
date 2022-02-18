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
pub struct IntegrationExport {
    #[serde(rename = "integration")]
    pub integration: Box<crate::models::DomainEntityRef>,
    /// True if the policy should export screen recordings in addition to the other conversation media. Default = true
    #[serde(rename = "shouldExportScreenRecordings", skip_serializing_if = "Option::is_none")]
    pub should_export_screen_recordings: Option<bool>,
}

impl IntegrationExport {
    pub fn new(integration: crate::models::DomainEntityRef) -> IntegrationExport {
        IntegrationExport {
            integration: Box::new(integration),
            should_export_screen_recordings: None,
        }
    }
}


