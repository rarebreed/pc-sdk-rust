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
pub struct CoachingAnnotationCreateRequest {
    /// The text of the annotation.
    #[serde(rename = "text")]
    pub text: String,
    /// Determines the permissions required to view this item.
    #[serde(rename = "accessType")]
    pub access_type: AccessType,
}

impl CoachingAnnotationCreateRequest {
    pub fn new(text: String, access_type: AccessType) -> CoachingAnnotationCreateRequest {
        CoachingAnnotationCreateRequest {
            text,
            access_type,
        }
    }
}

/// Determines the permissions required to view this item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccessType {
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "Private")]
    Private,
}

impl Default for AccessType {
    fn default() -> AccessType {
        Self::Public
    }
}

