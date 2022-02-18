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
pub struct AtzmTimeSlotWithTimeZone {
    /// The earliest time to dial a contact. Valid format is HH:mm
    #[serde(rename = "earliestCallableTime", skip_serializing_if = "Option::is_none")]
    pub earliest_callable_time: Option<String>,
    /// The latest time to dial a contact. Valid format is HH:mm
    #[serde(rename = "latestCallableTime", skip_serializing_if = "Option::is_none")]
    pub latest_callable_time: Option<String>,
    /// The time zone to use for contacts that cannot be mapped.
    #[serde(rename = "timeZoneId", skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
}

impl AtzmTimeSlotWithTimeZone {
    pub fn new() -> AtzmTimeSlotWithTimeZone {
        AtzmTimeSlotWithTimeZone {
            earliest_callable_time: None,
            latest_callable_time: None,
            time_zone_id: None,
        }
    }
}


