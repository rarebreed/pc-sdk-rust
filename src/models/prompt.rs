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
pub struct Prompt {
    /// The prompt identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The prompt name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of resources associated with this prompt
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<crate::models::PromptAsset>>,
    #[serde(rename = "currentOperation", skip_serializing_if = "Option::is_none")]
    pub current_operation: Option<Box<crate::models::Operation>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Prompt {
    pub fn new(name: String) -> Prompt {
        Prompt {
            id: None,
            name,
            description: None,
            resources: None,
            current_operation: None,
            self_uri: None,
        }
    }
}


