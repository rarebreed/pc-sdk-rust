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
pub struct ValidateAssignUsers {
    /// List of user ids to assign to a performance profile
    #[serde(rename = "membersToAssign")]
    pub members_to_assign: Vec<String>,
}

impl ValidateAssignUsers {
    pub fn new(members_to_assign: Vec<String>) -> ValidateAssignUsers {
        ValidateAssignUsers {
            members_to_assign,
        }
    }
}


