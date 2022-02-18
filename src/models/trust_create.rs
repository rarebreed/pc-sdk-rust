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
pub struct TrustCreate {
    /// The pairing Id created by the trustee. This is required to prove that the trustee agrees to the relationship.  Not required when creating a default pairing with Customer Care.
    #[serde(rename = "pairingId", skip_serializing_if = "Option::is_none")]
    pub pairing_id: Option<String>,
    /// If disabled no trustee user will have access, even if they were previously added.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The list of users and their roles to which access will be granted. The users are from the trustee and the roles are from the trustor. If no users are specified, at least one group is required.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::TrustMemberCreate>>,
    /// The list of groups and their roles to which access will be granted. The groups are from the trustee and the roles are from the trustor. If no groups are specified, at least one user is required.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::TrustMemberCreate>>,
    /// The expiration date of the trust. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateExpired", skip_serializing_if = "Option::is_none")]
    pub date_expired: Option<String>,
}

impl TrustCreate {
    pub fn new(enabled: bool) -> TrustCreate {
        TrustCreate {
            pairing_id: None,
            enabled,
            users: None,
            groups: None,
            date_expired: None,
        }
    }
}


