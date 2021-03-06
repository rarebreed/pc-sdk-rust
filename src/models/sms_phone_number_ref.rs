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
pub struct SmsPhoneNumberRef {
    /// A phone number provisioned for SMS communications in E.164 format. E.g. +13175555555 or +34234234234
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl SmsPhoneNumberRef {
    pub fn new(phone_number: String) -> SmsPhoneNumberRef {
        SmsPhoneNumberRef {
            phone_number,
            self_uri: None,
        }
    }
}


