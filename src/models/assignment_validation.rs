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
pub struct AssignmentValidation {
    /// The list of users that are not assigned to any custom performance profile
    #[serde(rename = "membersNotAssigned", skip_serializing_if = "Option::is_none")]
    pub members_not_assigned: Option<Vec<crate::models::UserReference>>,
    /// The list of users that are already assigned to the requesting custom performance profile
    #[serde(rename = "membersAlreadyAssigned", skip_serializing_if = "Option::is_none")]
    pub members_already_assigned: Option<Vec<crate::models::UserReference>>,
    /// The list of users that are already assigned to other custom performance profiles
    #[serde(rename = "membersAlreadyAssignedToOther", skip_serializing_if = "Option::is_none")]
    pub members_already_assigned_to_other: Option<Vec<crate::models::OtherProfileAssignment>>,
    /// The list of user id that are invalid for the gamfication service to handle
    #[serde(rename = "invalidMemberAssignments", skip_serializing_if = "Option::is_none")]
    pub invalid_member_assignments: Option<Vec<crate::models::InvalidAssignment>>,
}

impl AssignmentValidation {
    pub fn new() -> AssignmentValidation {
        AssignmentValidation {
            members_not_assigned: None,
            members_already_assigned: None,
            members_already_assigned_to_other: None,
            invalid_member_assignments: None,
        }
    }
}


