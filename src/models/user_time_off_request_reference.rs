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
pub struct UserTimeOffRequestReference {
    /// The id of the time off request
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "user")]
    pub user: Box<crate::models::UserReference>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl UserTimeOffRequestReference {
    pub fn new(user: crate::models::UserReference) -> UserTimeOffRequestReference {
        UserTimeOffRequestReference {
            id: None,
            user: Box::new(user),
            self_uri: None,
        }
    }
}

