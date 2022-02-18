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
pub struct DisallowedEntityLearningAssignmentReference {
    /// The error code associated with this disallowed entity
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<crate::models::LearningAssignmentReference>>,
}

impl DisallowedEntityLearningAssignmentReference {
    pub fn new() -> DisallowedEntityLearningAssignmentReference {
        DisallowedEntityLearningAssignmentReference {
            error_code: None,
            entity: None,
        }
    }
}


