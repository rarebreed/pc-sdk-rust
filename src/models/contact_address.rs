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
pub struct ContactAddress {
    #[serde(rename = "address1", skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl ContactAddress {
    pub fn new() -> ContactAddress {
        ContactAddress {
            address1: None,
            address2: None,
            city: None,
            state: None,
            postal_code: None,
            country_code: None,
        }
    }
}


