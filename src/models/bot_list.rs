/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// BotList : A list of BotConnectorBots



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotList {
    /// A list of botConnector Bots. Max 50
    #[serde(rename = "chatBots")]
    pub chat_bots: Vec<crate::models::BotConnectorBot>,
}

impl BotList {
    /// A list of BotConnectorBots
    pub fn new(chat_bots: Vec<crate::models::BotConnectorBot>) -> BotList {
        BotList {
            chat_bots,
        }
    }
}


