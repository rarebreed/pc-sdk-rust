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
pub struct LocalEncryptionConfiguration {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The url for decryption. This must specify the path to where Purecloud can requests decryption
    #[serde(rename = "url")]
    pub url: String,
    /// The api id for Hawk Authentication.
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// The api shared symmetric key used for hawk authentication
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl LocalEncryptionConfiguration {
    pub fn new(url: String, api_id: String, api_key: String) -> LocalEncryptionConfiguration {
        LocalEncryptionConfiguration {
            id: None,
            name: None,
            url,
            api_id,
            api_key,
            self_uri: None,
        }
    }
}

