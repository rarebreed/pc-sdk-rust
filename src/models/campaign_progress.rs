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
pub struct CampaignProgress {
    #[serde(rename = "campaign")]
    pub campaign: Box<crate::models::DomainEntityRef>,
    #[serde(rename = "contactList")]
    pub contact_list: Box<crate::models::DomainEntityRef>,
    /// Number of contacts called during the campaign
    #[serde(rename = "numberOfContactsCalled", skip_serializing_if = "Option::is_none")]
    pub number_of_contacts_called: Option<i64>,
    /// Number of contacts messaged during the campaign
    #[serde(rename = "numberOfContactsMessaged", skip_serializing_if = "Option::is_none")]
    pub number_of_contacts_messaged: Option<i64>,
    /// Total number of contacts in the campaign
    #[serde(rename = "totalNumberOfContacts", skip_serializing_if = "Option::is_none")]
    pub total_number_of_contacts: Option<i64>,
    /// Percentage of contacts processed during the campaign
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i64>,
}

impl CampaignProgress {
    pub fn new(campaign: crate::models::DomainEntityRef, contact_list: crate::models::DomainEntityRef) -> CampaignProgress {
        CampaignProgress {
            campaign: Box::new(campaign),
            contact_list: Box::new(contact_list),
            number_of_contacts_called: None,
            number_of_contacts_messaged: None,
            total_number_of_contacts: None,
            percentage: None,
        }
    }
}


