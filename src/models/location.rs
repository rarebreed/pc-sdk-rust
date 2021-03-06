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
pub struct Location {
    /// Unique identifier for the location
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique identifier for the location floorplan image
    #[serde(rename = "floorplanId", skip_serializing_if = "Option::is_none")]
    pub floorplan_id: Option<String>,
    /// Users coordinates on the floorplan. Only used when floorplanImage is set
    #[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<::std::collections::HashMap<String, f64>>,
    /// Optional description on the users location
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "locationDefinition", skip_serializing_if = "Option::is_none")]
    pub location_definition: Option<Box<crate::models::LocationDefinition>>,
}

impl Location {
    pub fn new() -> Location {
        Location {
            id: None,
            floorplan_id: None,
            coordinates: None,
            notes: None,
            location_definition: None,
        }
    }
}


