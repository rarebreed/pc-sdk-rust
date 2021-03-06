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
pub struct AnalyticsReportingSettings {
    /// Indication of whether or not personal data is masked in data export and the Analytics/Reporting UI
    #[serde(rename = "piiMaskingEnabled", skip_serializing_if = "Option::is_none")]
    pub pii_masking_enabled: Option<bool>,
    /// Indication of whether or not to obfuscate export data from the Queue Agent Details view based on User ACL
    #[serde(rename = "queueAgentAccessObfuscation", skip_serializing_if = "Option::is_none")]
    pub queue_agent_access_obfuscation: Option<bool>,
}

impl AnalyticsReportingSettings {
    pub fn new() -> AnalyticsReportingSettings {
        AnalyticsReportingSettings {
            pii_masking_enabled: None,
            queue_agent_access_obfuscation: None,
        }
    }
}


