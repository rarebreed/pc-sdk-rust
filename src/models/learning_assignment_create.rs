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
pub struct LearningAssignmentCreate {
    /// The Learning module Id associated with this assignment
    #[serde(rename = "moduleId")]
    pub module_id: String,
    /// The User for whom the assignment is assigned
    #[serde(rename = "userId")]
    pub user_id: String,
    /// The recommended completion date of assignment. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "recommendedCompletionDate", skip_serializing_if = "Option::is_none")]
    pub recommended_completion_date: Option<String>,
}

impl LearningAssignmentCreate {
    pub fn new(module_id: String, user_id: String) -> LearningAssignmentCreate {
        LearningAssignmentCreate {
            module_id,
            user_id,
            recommended_completion_date: None,
        }
    }
}


