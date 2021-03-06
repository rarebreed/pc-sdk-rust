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
pub struct TrustUpdate {
    /// If disabled no trustee user will have access, even if they were previously added.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The expiration date of the trust. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateExpired", skip_serializing_if = "Option::is_none")]
    pub date_expired: Option<String>,
}

impl TrustUpdate {
    pub fn new(enabled: bool) -> TrustUpdate {
        TrustUpdate {
            enabled,
            date_expired: None,
        }
    }
}


