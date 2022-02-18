/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// BotSlot : Description of a data value returned from an intent



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotSlot {
    /// The name of the slot. This can be up to 100 characters long and must be comprised of displayable characters without leading or trailing whitespace
    #[serde(rename = "name")]
    pub name: String,
    /// The data type of the slot string, integer, decimal, duration, boolean, currency, datetime or the xxxCollection versions of those types
    #[serde(rename = "type")]
    pub _type: String,
}

impl BotSlot {
    /// Description of a data value returned from an intent
    pub fn new(name: String, _type: String) -> BotSlot {
        BotSlot {
            name,
            _type,
        }
    }
}


