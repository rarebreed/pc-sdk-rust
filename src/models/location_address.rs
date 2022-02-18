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
pub struct LocationAddress {
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "countryName", skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "street1", skip_serializing_if = "Option::is_none")]
    pub street1: Option<String>,
    #[serde(rename = "street2", skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
    #[serde(rename = "zipcode", skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
}

impl LocationAddress {
    pub fn new() -> LocationAddress {
        LocationAddress {
            city: None,
            country: None,
            country_name: None,
            state: None,
            street1: None,
            street2: None,
            zipcode: None,
        }
    }
}

