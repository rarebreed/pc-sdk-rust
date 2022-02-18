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
pub struct TimeZoneMappingPreview {
    #[serde(rename = "contactList", skip_serializing_if = "Option::is_none")]
    pub contact_list: Option<Box<crate::models::DomainEntityRef>>,
    /// The number of contacts per time zone that mapped to only that time zone
    #[serde(rename = "contactsPerTimeZone", skip_serializing_if = "Option::is_none")]
    pub contacts_per_time_zone: Option<::std::collections::HashMap<String, i64>>,
    /// The number of contacts per time zone that mapped to only that time zone and were mapped using the zip code column
    #[serde(rename = "contactsMappedUsingZipCode", skip_serializing_if = "Option::is_none")]
    pub contacts_mapped_using_zip_code: Option<::std::collections::HashMap<String, i64>>,
    /// The total number of contacts that mapped to a single time zone
    #[serde(rename = "contactsMappedToASingleZone", skip_serializing_if = "Option::is_none")]
    pub contacts_mapped_to_a_single_zone: Option<i64>,
    /// The total number of contacts that mapped to a single time zone and were mapped using the zip code column
    #[serde(rename = "contactsMappedToASingleZoneUsingZipCode", skip_serializing_if = "Option::is_none")]
    pub contacts_mapped_to_a_single_zone_using_zip_code: Option<i64>,
    /// The total number of contacts that mapped to multiple time zones
    #[serde(rename = "contactsMappedToMultipleZones", skip_serializing_if = "Option::is_none")]
    pub contacts_mapped_to_multiple_zones: Option<i64>,
    /// The total number of contacts that mapped to multiple time zones and were mapped using the zip code column
    #[serde(rename = "contactsMappedToMultipleZonesUsingZipCode", skip_serializing_if = "Option::is_none")]
    pub contacts_mapped_to_multiple_zones_using_zip_code: Option<i64>,
    /// The total number of contacts that will be dialed during the default window
    #[serde(rename = "contactsInDefaultWindow", skip_serializing_if = "Option::is_none")]
    pub contacts_in_default_window: Option<i64>,
    /// The total number of contacts in the contact list
    #[serde(rename = "contactListSize", skip_serializing_if = "Option::is_none")]
    pub contact_list_size: Option<i64>,
}

impl TimeZoneMappingPreview {
    pub fn new() -> TimeZoneMappingPreview {
        TimeZoneMappingPreview {
            contact_list: None,
            contacts_per_time_zone: None,
            contacts_mapped_using_zip_code: None,
            contacts_mapped_to_a_single_zone: None,
            contacts_mapped_to_a_single_zone_using_zip_code: None,
            contacts_mapped_to_multiple_zones: None,
            contacts_mapped_to_multiple_zones_using_zip_code: None,
            contacts_in_default_window: None,
            contact_list_size: None,
        }
    }
}


