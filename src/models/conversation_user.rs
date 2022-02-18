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
pub struct ConversationUser {
    /// The globally unique identifier for this user.
    #[serde(rename = "id")]
    pub id: String,
}

impl ConversationUser {
    pub fn new(id: String) -> ConversationUser {
        ConversationUser {
            id,
        }
    }
}

