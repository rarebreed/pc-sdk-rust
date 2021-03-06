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
pub struct Note {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of the contact or organization to which this note refers. This only needs to be set for input when using the Bulk APIs.
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// This is only need to be set when using Bulk API. Using any other value than contact or organization will result in null being used.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    #[serde(rename = "noteText", skip_serializing_if = "Option::is_none")]
    pub note_text: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "modifyDate", skip_serializing_if = "Option::is_none")]
    pub modify_date: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "createdBy")]
    pub created_by: Box<crate::models::User>,
    /// Links to the sources of data (e.g. one source might be a CRM) that contributed data to this record.  Read-only, and only populated when requested via expand param.
    #[serde(rename = "externalDataSources", skip_serializing_if = "Option::is_none")]
    pub external_data_sources: Option<Vec<crate::models::ExternalDataSource>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Note {
    pub fn new(created_by: crate::models::User) -> Note {
        Note {
            id: None,
            entity_id: None,
            entity_type: None,
            note_text: None,
            modify_date: None,
            create_date: None,
            created_by: Box::new(created_by),
            external_data_sources: None,
            self_uri: None,
        }
    }
}

/// This is only need to be set when using Bulk API. Using any other value than contact or organization will result in null being used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "contact")]
    Contact,
    #[serde(rename = "organization")]
    Organization,
}

impl Default for EntityType {
    fn default() -> EntityType {
        Self::Contact
    }
}

