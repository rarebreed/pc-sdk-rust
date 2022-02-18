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
pub struct Phone {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the entity.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::Division>>,
    /// The resource's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The current version of the resource.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The date the resource was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date of the last modification to the resource. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// The ID of the user that last modified the resource.
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// The ID of the user that created the resource.
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Indicates if the resource is active, inactive, or deleted.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The application that last modified the resource.
    #[serde(rename = "modifiedByApp", skip_serializing_if = "Option::is_none")]
    pub modified_by_app: Option<String>,
    /// The application that created the resource.
    #[serde(rename = "createdByApp", skip_serializing_if = "Option::is_none")]
    pub created_by_app: Option<String>,
    #[serde(rename = "site")]
    pub site: Box<crate::models::DomainEntityRef>,
    #[serde(rename = "phoneBaseSettings")]
    pub phone_base_settings: Box<crate::models::DomainEntityRef>,
    #[serde(rename = "lineBaseSettings", skip_serializing_if = "Option::is_none")]
    pub line_base_settings: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "phoneMetaBase", skip_serializing_if = "Option::is_none")]
    pub phone_meta_base: Option<Box<crate::models::DomainEntityRef>>,
    /// Lines
    #[serde(rename = "lines")]
    pub lines: Vec<crate::models::Line>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::PhoneStatus>>,
    #[serde(rename = "secondaryStatus", skip_serializing_if = "Option::is_none")]
    pub secondary_status: Option<Box<crate::models::PhoneStatus>>,
    #[serde(rename = "userAgentInfo", skip_serializing_if = "Option::is_none")]
    pub user_agent_info: Option<Box<crate::models::UserAgentInfo>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Box<crate::models::PhoneCapabilities>>,
    #[serde(rename = "webRtcUser", skip_serializing_if = "Option::is_none")]
    pub web_rtc_user: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "primaryEdge", skip_serializing_if = "Option::is_none")]
    pub primary_edge: Option<Box<crate::models::Edge>>,
    #[serde(rename = "secondaryEdge", skip_serializing_if = "Option::is_none")]
    pub secondary_edge: Option<Box<crate::models::Edge>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Phone {
    pub fn new(name: String, site: crate::models::DomainEntityRef, phone_base_settings: crate::models::DomainEntityRef, lines: Vec<crate::models::Line>) -> Phone {
        Phone {
            id: None,
            name,
            division: None,
            description: None,
            version: None,
            date_created: None,
            date_modified: None,
            modified_by: None,
            created_by: None,
            state: None,
            modified_by_app: None,
            created_by_app: None,
            site: Box::new(site),
            phone_base_settings: Box::new(phone_base_settings),
            line_base_settings: None,
            phone_meta_base: None,
            lines,
            status: None,
            secondary_status: None,
            user_agent_info: None,
            properties: None,
            capabilities: None,
            web_rtc_user: None,
            primary_edge: None,
            secondary_edge: None,
            self_uri: None,
        }
    }
}

/// Indicates if the resource is active, inactive, or deleted.
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
