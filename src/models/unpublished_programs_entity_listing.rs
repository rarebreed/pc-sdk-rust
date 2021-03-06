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
pub struct UnpublishedProgramsEntityListing {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::Program>>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    #[serde(rename = "nextUri", skip_serializing_if = "Option::is_none")]
    pub next_uri: Option<String>,
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
}

impl UnpublishedProgramsEntityListing {
    pub fn new() -> UnpublishedProgramsEntityListing {
        UnpublishedProgramsEntityListing {
            entities: None,
            page_size: None,
            self_uri: None,
            next_uri: None,
            page_count: None,
        }
    }
}


