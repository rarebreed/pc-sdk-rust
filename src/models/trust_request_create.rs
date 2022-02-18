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
pub struct TrustRequestCreate {
    /// The list of trustee users that are requesting access. If no users are specified, at least one group is required.
    #[serde(rename = "userIds", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// The list of trustee groups that are requesting access. If no groups are specified, at least one user is required.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

impl TrustRequestCreate {
    pub fn new() -> TrustRequestCreate {
        TrustRequestCreate {
            user_ids: None,
            group_ids: None,
        }
    }
}


