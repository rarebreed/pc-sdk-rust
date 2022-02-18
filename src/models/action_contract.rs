/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ActionContract : This resource contains all of the schemas needed to define the inputs and outputs, of a single Action.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionContract {
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<Box<crate::models::ActionOutput>>,
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<Box<crate::models::ActionInput>>,
}

impl ActionContract {
    /// This resource contains all of the schemas needed to define the inputs and outputs, of a single Action.
    pub fn new() -> ActionContract {
        ActionContract {
            output: None,
            input: None,
        }
    }
}

