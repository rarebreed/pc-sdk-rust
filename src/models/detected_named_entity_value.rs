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
pub struct DetectedNamedEntityValue {
    /// The raw value of the detected named entity.
    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    /// The resolved value of the detected named entity.
    #[serde(rename = "resolved", skip_serializing_if = "Option::is_none")]
    pub resolved: Option<String>,
}

impl DetectedNamedEntityValue {
    pub fn new() -> DetectedNamedEntityValue {
        DetectedNamedEntityValue {
            raw: None,
            resolved: None,
        }
    }
}


