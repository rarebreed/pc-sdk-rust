/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// PostInputContract : The schemas defining all of the expected requests/inputs.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostInputContract {
    #[serde(rename = "inputSchema")]
    pub input_schema: Box<crate::models::JsonSchemaDocument>,
}

impl PostInputContract {
    /// The schemas defining all of the expected requests/inputs.
    pub fn new(input_schema: crate::models::JsonSchemaDocument) -> PostInputContract {
        PostInputContract {
            input_schema: Box::new(input_schema),
        }
    }
}


