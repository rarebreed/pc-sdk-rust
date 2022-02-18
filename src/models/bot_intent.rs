/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// BotIntent : A botConnector's bot intention



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotIntent {
    /// The name of this intent.  This can be up to 100 characters long and must be comprised of displayable characters without leading or trailing whitespace
    #[serde(rename = "name")]
    pub name: String,
    /// Optional returned data values associated with this intent, limit of 50.
    #[serde(rename = "slots", skip_serializing_if = "Option::is_none")]
    pub slots: Option<::std::collections::HashMap<String, crate::models::BotSlot>>,
}

impl BotIntent {
    /// A botConnector's bot intention
    pub fn new(name: String) -> BotIntent {
        BotIntent {
            name,
            slots: None,
        }
    }
}

