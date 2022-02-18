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
pub struct CreateShareResponse {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sharedEntityType", skip_serializing_if = "Option::is_none")]
    pub shared_entity_type: Option<SharedEntityType>,
    #[serde(rename = "sharedEntity", skip_serializing_if = "Option::is_none")]
    pub shared_entity: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "memberType", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<MemberType>,
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "sharedBy", skip_serializing_if = "Option::is_none")]
    pub shared_by: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "workspace", skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "succeeded", skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<Vec<crate::models::Share>>,
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<crate::models::Share>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl CreateShareResponse {
    pub fn new() -> CreateShareResponse {
        CreateShareResponse {
            id: None,
            name: None,
            shared_entity_type: None,
            shared_entity: None,
            member_type: None,
            member: None,
            shared_by: None,
            workspace: None,
            succeeded: None,
            failed: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SharedEntityType {
    #[serde(rename = "DOCUMENT")]
    DOCUMENT,
}

impl Default for SharedEntityType {
    fn default() -> SharedEntityType {
        Self::DOCUMENT
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MemberType {
    #[serde(rename = "USER")]
    USER,
    #[serde(rename = "GROUP")]
    GROUP,
    #[serde(rename = "PUBLIC")]
    PUBLIC,
}

impl Default for MemberType {
    fn default() -> MemberType {
        Self::USER
    }
}
