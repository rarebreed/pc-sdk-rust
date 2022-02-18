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
pub struct DialogflowIntent {
    /// The intent name
    #[serde(rename = "name")]
    pub name: String,
    /// An object mapping parameter names to Parameter objects
    #[serde(rename = "parameters")]
    pub parameters: ::std::collections::HashMap<String, crate::models::DialogflowParameter>,
}

impl DialogflowIntent {
    pub fn new(name: String, parameters: ::std::collections::HashMap<String, crate::models::DialogflowParameter>) -> DialogflowIntent {
        DialogflowIntent {
            name,
            parameters,
        }
    }
}


