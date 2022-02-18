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
pub struct OAuthClientListing {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the OAuth client.
    #[serde(rename = "name")]
    pub name: String,
    /// The number of seconds, between 5mins and 48hrs, until tokens created with this client expire. If this field is omitted, a default of 24 hours will be applied.
    #[serde(rename = "accessTokenValiditySeconds", skip_serializing_if = "Option::is_none")]
    pub access_token_validity_seconds: Option<i64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of allowed callbacks for this client. For example: https://myap.example.com/auth/callback
    #[serde(rename = "registeredRedirectUri", skip_serializing_if = "Option::is_none")]
    pub registered_redirect_uri: Option<Vec<String>>,
    /// System created secret assigned to this client. Secrets are required for code authorization and client credential grants.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Deprecated. Use roleDivisions instead.
    #[serde(rename = "roleIds", skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    /// Date this client was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date this client was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::DomainEntityRef>>,
    /// The scope requested by this client. Scopes only apply to clients not using the client_credential grant
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    /// Set of roles and their corresponding divisions associated with this client. Roles and divisions only apply to clients using the client_credential grant
    #[serde(rename = "roleDivisions", skip_serializing_if = "Option::is_none")]
    pub role_divisions: Option<Vec<crate::models::RoleDivision>>,
    /// The state of the OAuth client. Active: The OAuth client can be used to create access tokens. This is the default state. Disabled: Access tokens created by the client are invalid and new ones cannot be created. Inactive: Access tokens cannot be created with this OAuth client and it will be deleted.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The time at which this client will be deleted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateToDelete", skip_serializing_if = "Option::is_none")]
    pub date_to_delete: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl OAuthClientListing {
    pub fn new(name: String) -> OAuthClientListing {
        OAuthClientListing {
            id: None,
            name,
            access_token_validity_seconds: None,
            description: None,
            registered_redirect_uri: None,
            secret: None,
            role_ids: None,
            date_created: None,
            date_modified: None,
            created_by: None,
            modified_by: None,
            scope: None,
            role_divisions: None,
            state: None,
            date_to_delete: None,
            self_uri: None,
        }
    }
}

/// The state of the OAuth client. Active: The OAuth client can be used to create access tokens. This is the default state. Disabled: Access tokens created by the client are invalid and new ones cannot be created. Inactive: Access tokens cannot be created with this OAuth client and it will be deleted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}

