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
pub struct InboundDomain {
    /// Unique Id of the domain such as: example.com
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Mx Record Status
    #[serde(rename = "mxRecordStatus")]
    pub mx_record_status: MxRecordStatus,
    /// Indicates if this a PureCloud sub-domain.  If true, then the appropriate DNS records are created for sending/receiving email.
    #[serde(rename = "subDomain", skip_serializing_if = "Option::is_none")]
    pub sub_domain: Option<bool>,
    #[serde(rename = "mailFromSettings", skip_serializing_if = "Option::is_none")]
    pub mail_from_settings: Option<Box<crate::models::MailFromResult>>,
    #[serde(rename = "customSMTPServer", skip_serializing_if = "Option::is_none")]
    pub custom_smtp_server: Option<Box<crate::models::DomainEntityRef>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl InboundDomain {
    pub fn new(mx_record_status: MxRecordStatus) -> InboundDomain {
        InboundDomain {
            id: None,
            name: None,
            mx_record_status,
            sub_domain: None,
            mail_from_settings: None,
            custom_smtp_server: None,
            self_uri: None,
        }
    }
}

/// Mx Record Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MxRecordStatus {
    #[serde(rename = "VALID")]
    VALID,
    #[serde(rename = "INVALID")]
    INVALID,
    #[serde(rename = "NOT_AVAILABLE")]
    NOTAVAILABLE,
}

impl Default for MxRecordStatus {
    fn default() -> MxRecordStatus {
        Self::VALID
    }
}

