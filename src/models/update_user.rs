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
pub struct UpdateUser {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<Box<crate::models::Chat>>,
    #[serde(rename = "department", skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The address(s) used for primary contact. Updates to the corresponding address in the addresses list will be reflected here.
    #[serde(rename = "primaryContactInfo", skip_serializing_if = "Option::is_none")]
    pub primary_contact_info: Option<Vec<crate::models::Contact>>,
    /// Email address, phone number, and/or extension for this user. One entry is allowed per media type
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<crate::models::Contact>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "manager", skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::UserImage>>,
    /// This value should be the current version of the user. The current version can be obtained with a GET on the user before doing a PATCH.
    #[serde(rename = "version")]
    pub version: i32,
    /// Profile skills possessed by the user
    #[serde(rename = "profileSkills", skip_serializing_if = "Option::is_none")]
    pub profile_skills: Option<Vec<String>>,
    /// The user placement at each site location.
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<crate::models::Location>>,
    /// The groups the user is a member of
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::Group>>,
    /// The state of the user. This property can be used to restore a deleted user or transition between active and inactive. If specified, it is the only modifiable field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The value that denotes if acdAutoAnswer is set on the user
    #[serde(rename = "acdAutoAnswer", skip_serializing_if = "Option::is_none")]
    pub acd_auto_answer: Option<bool>,
    #[serde(rename = "certifications", skip_serializing_if = "Option::is_none")]
    pub certifications: Option<Vec<String>>,
    #[serde(rename = "biography", skip_serializing_if = "Option::is_none")]
    pub biography: Option<Box<crate::models::Biography>>,
    #[serde(rename = "employerInfo", skip_serializing_if = "Option::is_none")]
    pub employer_info: Option<Box<crate::models::EmployerInfo>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl UpdateUser {
    pub fn new(version: i32) -> UpdateUser {
        UpdateUser {
            id: None,
            name: None,
            chat: None,
            department: None,
            email: None,
            primary_contact_info: None,
            addresses: None,
            title: None,
            username: None,
            manager: None,
            images: None,
            version,
            profile_skills: None,
            locations: None,
            groups: None,
            state: None,
            acd_auto_answer: None,
            certifications: None,
            biography: None,
            employer_info: None,
            self_uri: None,
        }
    }
}

/// The state of the user. This property can be used to restore a deleted user or transition between active and inactive. If specified, it is the only modifiable field.
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

