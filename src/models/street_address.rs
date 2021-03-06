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
pub struct StreetAddress {
    /// 2 Letter Country code, like US or GB
    #[serde(rename = "country")]
    pub country: String,
    /// State or Province
    #[serde(rename = "A1")]
    pub a1: String,
    /// City or township
    #[serde(rename = "A3")]
    pub a3: String,
    /// Number and street
    #[serde(rename = "RD")]
    pub RD: String,
    /// House Number
    #[serde(rename = "HNO")]
    pub HNO: String,
    /// extra location info like suite 300
    #[serde(rename = "LOC", skip_serializing_if = "Option::is_none")]
    pub LOC: Option<String>,
    /// Name of the customer
    #[serde(rename = "NAM", skip_serializing_if = "Option::is_none")]
    pub NAM: Option<String>,
    /// Postal code
    #[serde(rename = "PC")]
    pub PC: String,
}

impl StreetAddress {
    pub fn new(country: String, a1: String, a3: String, RD: String, HNO: String, PC: String) -> StreetAddress {
        StreetAddress {
            country,
            a1,
            a3,
            RD,
            HNO,
            LOC: None,
            NAM: None,
            PC,
        }
    }
}


