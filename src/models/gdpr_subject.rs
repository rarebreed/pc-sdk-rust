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
pub struct GdprSubject {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "externalContactId", skip_serializing_if = "Option::is_none")]
    pub external_contact_id: Option<String>,
    #[serde(rename = "dialerContactId", skip_serializing_if = "Option::is_none")]
    pub dialer_contact_id: Option<Box<crate::models::DialerContactId>>,
    #[serde(rename = "journeyCustomer", skip_serializing_if = "Option::is_none")]
    pub journey_customer: Option<Box<crate::models::GdprJourneyCustomer>>,
    #[serde(rename = "socialHandle", skip_serializing_if = "Option::is_none")]
    pub social_handle: Option<Box<crate::models::SocialHandle>>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(rename = "phoneNumbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
    #[serde(rename = "emailAddresses", skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<String>>,
}

impl GdprSubject {
    pub fn new() -> GdprSubject {
        GdprSubject {
            name: None,
            user_id: None,
            external_contact_id: None,
            dialer_contact_id: None,
            journey_customer: None,
            social_handle: None,
            external_id: None,
            addresses: None,
            phone_numbers: None,
            email_addresses: None,
        }
    }
}


