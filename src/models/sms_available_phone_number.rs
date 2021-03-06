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
pub struct SmsAvailablePhoneNumber {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A phone number available for provisioning in E.164 format. E.g. +13175555555 or +34234234234
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The ISO 3166-1 alpha-2 country code of the country this phone number is associated with.
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The region/province/state the phone number is associated with.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The city the phone number is associated with.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The capabilities of the phone number available for provisioning.
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<Capabilities>>,
    /// The type of phone number available for provisioning.
    #[serde(rename = "phoneNumberType", skip_serializing_if = "Option::is_none")]
    pub phone_number_type: Option<PhoneNumberType>,
    /// The address requirement needed for provisioning this number. If there is a requirement, the address must be the residence or place of business of the individual or entity using the phone number.
    #[serde(rename = "addressRequirement", skip_serializing_if = "Option::is_none")]
    pub address_requirement: Option<AddressRequirement>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl SmsAvailablePhoneNumber {
    pub fn new() -> SmsAvailablePhoneNumber {
        SmsAvailablePhoneNumber {
            id: None,
            name: None,
            phone_number: None,
            country_code: None,
            region: None,
            city: None,
            capabilities: None,
            phone_number_type: None,
            address_requirement: None,
            self_uri: None,
        }
    }
}

/// The capabilities of the phone number available for provisioning.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Capabilities {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "mms")]
    Mms,
    #[serde(rename = "voice")]
    Voice,
}

impl Default for Capabilities {
    fn default() -> Capabilities {
        Self::Sms
    }
}
/// The type of phone number available for provisioning.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PhoneNumberType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "tollfree")]
    Tollfree,
    #[serde(rename = "shortcode")]
    Shortcode,
}

impl Default for PhoneNumberType {
    fn default() -> PhoneNumberType {
        Self::Local
    }
}
/// The address requirement needed for provisioning this number. If there is a requirement, the address must be the residence or place of business of the individual or entity using the phone number.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AddressRequirement {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "foreign")]
    Foreign,
}

impl Default for AddressRequirement {
    fn default() -> AddressRequirement {
        Self::None
    }
}

