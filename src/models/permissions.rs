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
pub struct Permissions {
    /// List of permission ids.
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

impl Permissions {
    pub fn new(ids: Vec<String>) -> Permissions {
        Permissions {
            ids,
        }
    }
}

