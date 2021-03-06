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
pub struct ReplaceRequest {
    #[serde(rename = "changeNumber", skip_serializing_if = "Option::is_none")]
    pub change_number: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "authToken", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
}

impl ReplaceRequest {
    pub fn new() -> ReplaceRequest {
        ReplaceRequest {
            change_number: None,
            name: None,
            auth_token: None,
        }
    }
}


