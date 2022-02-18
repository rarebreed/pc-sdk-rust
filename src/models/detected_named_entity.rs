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
pub struct DetectedNamedEntity {
    /// The name of the detected named entity.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the detected named entity.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// The probability of the detected named entity.
    #[serde(rename = "probability", skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<crate::models::DetectedNamedEntityValue>>,
}

impl DetectedNamedEntity {
    pub fn new() -> DetectedNamedEntity {
        DetectedNamedEntity {
            name: None,
            entity_type: None,
            probability: None,
            value: None,
        }
    }
}


