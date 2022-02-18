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
pub struct LocationAddressVerificationDetails {
    /// Status of address verification process
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Finished time of address verification process. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateFinished", skip_serializing_if = "Option::is_none")]
    pub date_finished: Option<String>,
    /// Time started of address verification process. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStarted", skip_serializing_if = "Option::is_none")]
    pub date_started: Option<String>,
    /// Third party service used for address verification
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl LocationAddressVerificationDetails {
    pub fn new() -> LocationAddressVerificationDetails {
        LocationAddressVerificationDetails {
            status: None,
            date_finished: None,
            date_started: None,
            service: None,
        }
    }
}

/// Status of address verification process
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Retry")]
    Retry,
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

