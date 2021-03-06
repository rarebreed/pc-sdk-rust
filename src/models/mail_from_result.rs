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
pub struct MailFromResult {
    /// The verification status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The list of DNS records that pertain that need to exist for verification.
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<crate::models::Record>>,
    /// The custom MAIL FROM domain.
    #[serde(rename = "mailFromDomain")]
    pub mail_from_domain: String,
}

impl MailFromResult {
    pub fn new(mail_from_domain: String) -> MailFromResult {
        MailFromResult {
            status: None,
            records: None,
            mail_from_domain,
        }
    }
}

/// The verification status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "FAILED")]
    FAILED,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "VERIFIED")]
    VERIFIED,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}

impl Default for Status {
    fn default() -> Status {
        Self::FAILED
    }
}

