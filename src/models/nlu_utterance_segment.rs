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
pub struct NluUtteranceSegment {
    /// The text of the segment.
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<crate::models::NamedEntityAnnotation>>,
}

impl NluUtteranceSegment {
    pub fn new(text: String) -> NluUtteranceSegment {
        NluUtteranceSegment {
            text,
            entity: None,
        }
    }
}

