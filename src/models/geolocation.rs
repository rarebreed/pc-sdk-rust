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
pub struct Geolocation {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A string used to describe the type of client the geolocation is being updated from e.g. ios, android, web, etc.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// A boolean used to tell whether or not to set this geolocation client as the primary on a PATCH
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<crate::models::LocationDefinition>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Geolocation {
    pub fn new() -> Geolocation {
        Geolocation {
            id: None,
            name: None,
            _type: None,
            primary: None,
            latitude: None,
            longitude: None,
            country: None,
            region: None,
            city: None,
            locations: None,
            self_uri: None,
        }
    }
}


