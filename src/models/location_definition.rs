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
pub struct LocationDefinition {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "contactUser", skip_serializing_if = "Option::is_none")]
    pub contact_user: Option<Box<crate::models::AddressableEntityRef>>,
    #[serde(rename = "emergencyNumber", skip_serializing_if = "Option::is_none")]
    pub emergency_number: Option<Box<crate::models::LocationEmergencyNumber>>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<crate::models::LocationAddress>>,
    /// Current state of the location entity
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Notes for the location entity
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Current version of the location entity, value to be supplied should be retrieved by a GET or on create/update response
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// A list of ancestor IDs in order
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    /// Profile image of the location entity, retrieved with ?expand=images query parameter
    #[serde(rename = "profileImage", skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<Vec<crate::models::LocationImage>>,
    /// Floorplan images of the location entity, retrieved with ?expand=images query parameter
    #[serde(rename = "floorplanImage", skip_serializing_if = "Option::is_none")]
    pub floorplan_image: Option<Vec<crate::models::LocationImage>>,
    #[serde(rename = "addressVerificationDetails", skip_serializing_if = "Option::is_none")]
    pub address_verification_details: Option<Box<crate::models::LocationAddressVerificationDetails>>,
    /// Boolean field which states if the address has been verified as an actual address
    #[serde(rename = "addressVerified", skip_serializing_if = "Option::is_none")]
    pub address_verified: Option<bool>,
    /// Boolean field which states if the address has been stored for E911
    #[serde(rename = "addressStored", skip_serializing_if = "Option::is_none")]
    pub address_stored: Option<bool>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl LocationDefinition {
    pub fn new() -> LocationDefinition {
        LocationDefinition {
            id: None,
            name: None,
            contact_user: None,
            emergency_number: None,
            address: None,
            state: None,
            notes: None,
            version: None,
            path: None,
            profile_image: None,
            floorplan_image: None,
            address_verification_details: None,
            address_verified: None,
            address_stored: None,
            images: None,
            self_uri: None,
        }
    }
}

/// Current state of the location entity
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}

