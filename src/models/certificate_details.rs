/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// CertificateDetails : Represents the details of a parsed certificate.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CertificateDetails {
    /// Information about the issuer of the certificate.  The value of this property is a comma separated key=value format.  Each key is one of the attribute names supported by X.500.
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Information about the subject of the certificate.  The value of this property is a comma separated key=value format.  Each key is one of the attribute names supported by X.500.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The expiration date of the certificate. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// The issue date of the certificate. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "issueDate", skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    /// True if the certificate is expired, false otherwise.
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(rename = "signatureValid", skip_serializing_if = "Option::is_none")]
    pub signature_valid: Option<bool>,
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

impl CertificateDetails {
    /// Represents the details of a parsed certificate.
    pub fn new() -> CertificateDetails {
        CertificateDetails {
            issuer: None,
            subject: None,
            expiration_date: None,
            issue_date: None,
            expired: None,
            signature_valid: None,
            valid: None,
        }
    }
}


