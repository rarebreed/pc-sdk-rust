/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ActionContractInput : Contract definition.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionContractInput {
    #[serde(rename = "input")]
    pub input: Box<crate::models::PostInputContract>,
    #[serde(rename = "output")]
    pub output: Box<crate::models::PostOutputContract>,
}

impl ActionContractInput {
    /// Contract definition.
    pub fn new(input: crate::models::PostInputContract, output: crate::models::PostOutputContract) -> ActionContractInput {
        ActionContractInput {
            input: Box::new(input),
            output: Box::new(output),
        }
    }
}


