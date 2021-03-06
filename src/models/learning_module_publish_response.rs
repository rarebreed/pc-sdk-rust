/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// LearningModulePublishResponse : Learning module publish response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LearningModulePublishResponse {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The version of published learning module
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl LearningModulePublishResponse {
    /// Learning module publish response
    pub fn new() -> LearningModulePublishResponse {
        LearningModulePublishResponse {
            id: None,
            version: None,
            self_uri: None,
        }
    }
}


