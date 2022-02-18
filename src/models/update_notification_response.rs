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
pub struct UpdateNotificationResponse {
    /// The mutableGroupId of the notification
    #[serde(rename = "mutableGroupId", skip_serializing_if = "Option::is_none")]
    pub mutable_group_id: Option<String>,
    /// The id of the notification for mapping the potentially new mutableGroupId
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateNotificationResponse {
    pub fn new() -> UpdateNotificationResponse {
        UpdateNotificationResponse {
            mutable_group_id: None,
            id: None,
        }
    }
}

