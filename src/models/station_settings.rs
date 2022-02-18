/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// StationSettings : Organization settings for stations



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StationSettings {
    #[serde(rename = "freeSeatingConfiguration", skip_serializing_if = "Option::is_none")]
    pub free_seating_configuration: Option<Box<crate::models::FreeSeatingConfiguration>>,
}

impl StationSettings {
    /// Organization settings for stations
    pub fn new() -> StationSettings {
        StationSettings {
            free_seating_configuration: None,
        }
    }
}


