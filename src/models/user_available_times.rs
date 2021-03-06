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
pub struct UserAvailableTimes {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::UserReference>>,
    /// Periods of availability to schedule coaching appointment for an user
    #[serde(rename = "availableTimes", skip_serializing_if = "Option::is_none")]
    pub available_times: Option<Vec<crate::models::AvailableTime>>,
}

impl UserAvailableTimes {
    pub fn new() -> UserAvailableTimes {
        UserAvailableTimes {
            user: None,
            available_times: None,
        }
    }
}


