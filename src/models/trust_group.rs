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
pub struct TrustGroup {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The group name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Last modified date/time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// Number of members.
    #[serde(rename = "memberCount", skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>,
    /// Active, inactive, or deleted state.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Current version for this resource.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// Type of group.
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::UserImage>>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<crate::models::GroupContact>>,
    /// Are membership rules visible to the person requesting to view the group
    #[serde(rename = "rulesVisible")]
    pub rules_visible: bool,
    /// Who can view this group
    #[serde(rename = "visibility")]
    pub visibility: Visibility,
    /// Owners of the group
    #[serde(rename = "owners", skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<crate::models::User>>,
    /// The date on which the trusted group was added. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::OrgUser>>,
}

impl TrustGroup {
    pub fn new(name: String, _type: Type, rules_visible: bool, visibility: Visibility) -> TrustGroup {
        TrustGroup {
            id: None,
            name,
            description: None,
            date_modified: None,
            member_count: None,
            state: None,
            version: None,
            _type,
            images: None,
            addresses: None,
            rules_visible,
            visibility,
            owners: None,
            date_created: None,
            created_by: None,
        }
    }
}

/// Active, inactive, or deleted state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}
/// Type of group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "official")]
    Official,
    #[serde(rename = "social")]
    Social,
}

impl Default for Type {
    fn default() -> Type {
        Self::Official
    }
}
/// Who can view this group
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "owners")]
    Owners,
    #[serde(rename = "members")]
    Members,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Public
    }
}

