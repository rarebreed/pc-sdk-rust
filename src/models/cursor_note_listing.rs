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
pub struct CursorNoteListing {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::Note>>,
    #[serde(rename = "nextUri", skip_serializing_if = "Option::is_none")]
    pub next_uri: Option<String>,
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    #[serde(rename = "previousUri", skip_serializing_if = "Option::is_none")]
    pub previous_uri: Option<String>,
}

impl CursorNoteListing {
    pub fn new() -> CursorNoteListing {
        CursorNoteListing {
            entities: None,
            next_uri: None,
            self_uri: None,
            previous_uri: None,
        }
    }
}

