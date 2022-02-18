/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// VoicemailRetentionPolicy : Governs how the voicemail is retained



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VoicemailRetentionPolicy {
    /// The retention policy type
    #[serde(rename = "voicemailRetentionPolicyType", skip_serializing_if = "Option::is_none")]
    pub voicemail_retention_policy_type: Option<VoicemailRetentionPolicyType>,
    /// If retentionPolicyType == RETAIN_WITH_TTL, then this value represents the number of days for the TTL
    #[serde(rename = "numberOfDays", skip_serializing_if = "Option::is_none")]
    pub number_of_days: Option<i32>,
}

impl VoicemailRetentionPolicy {
    /// Governs how the voicemail is retained
    pub fn new() -> VoicemailRetentionPolicy {
        VoicemailRetentionPolicy {
            voicemail_retention_policy_type: None,
            number_of_days: None,
        }
    }
}

/// The retention policy type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoicemailRetentionPolicyType {
    #[serde(rename = "RETAIN_INDEFINITELY")]
    RETAININDEFINITELY,
    #[serde(rename = "RETAIN_WITH_TTL")]
    RETAINWITHTTL,
    #[serde(rename = "IMMEDIATE_DELETE")]
    IMMEDIATEDELETE,
}

impl Default for VoicemailRetentionPolicyType {
    fn default() -> VoicemailRetentionPolicyType {
        Self::RETAININDEFINITELY
    }
}
