/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// DomainCertificateAuthority : A certificate authority represents an organization that has issued a digital certificate for making secure connections with an edge device.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainCertificateAuthority {
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
    /// The authorities signed X509 PEM encoded certificate.
    #[serde(rename = "certificate")]
    pub certificate: String,
    /// The certificate authorities type.  Managed certificate authorities are generated and maintained by Interactive Intelligence.  These are read-only and not modifiable by clients.  Remote authorities are customer managed.
    #[serde(rename = "type")]
    pub _type: Type,
    /// The service(s) that the authority can be used to authenticate.
    #[serde(rename = "services")]
    pub services: Vec<Services>,
    /// The details of the parsed certificate(s).
    #[serde(rename = "certificateDetails", skip_serializing_if = "Option::is_none")]
    pub certificate_details: Option<Vec<crate::models::CertificateDetails>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl DomainCertificateAuthority {
    /// A certificate authority represents an organization that has issued a digital certificate for making secure connections with an edge device.
    pub fn new(name: String, certificate: String, _type: Type, services: Vec<Services>) -> DomainCertificateAuthority {
        DomainCertificateAuthority {
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
            certificate,
            _type,
            services,
            certificate_details: None,
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
/// The certificate authorities type.  Managed certificate authorities are generated and maintained by Interactive Intelligence.  These are read-only and not modifiable by clients.  Remote authorities are customer managed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MANAGED")]
    MANAGED,
    #[serde(rename = "REMOTE")]
    REMOTE,
}

impl Default for Type {
    fn default() -> Type {
        Self::MANAGED
    }
}
/// The service(s) that the authority can be used to authenticate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Services {
    #[serde(rename = "SIP")]
    SIP,
    #[serde(rename = "PROVISION")]
    PROVISION,
    #[serde(rename = "PROVISION_PHONE")]
    PROVISIONPHONE,
}

impl Default for Services {
    fn default() -> Services {
        Self::SIP
    }
}
