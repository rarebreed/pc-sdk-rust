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
pub struct ChatMediaPolicy {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Box<crate::models::PolicyActions>>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Box<crate::models::ChatMediaPolicyConditions>>,
}

impl ChatMediaPolicy {
    pub fn new() -> ChatMediaPolicy {
        ChatMediaPolicy {
            actions: None,
            conditions: None,
        }
    }
}


