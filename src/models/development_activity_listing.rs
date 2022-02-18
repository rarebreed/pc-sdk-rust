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
pub struct DevelopmentActivityListing {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::DevelopmentActivity>>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "firstUri", skip_serializing_if = "Option::is_none")]
    pub first_uri: Option<String>,
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    #[serde(rename = "nextUri", skip_serializing_if = "Option::is_none")]
    pub next_uri: Option<String>,
    #[serde(rename = "previousUri", skip_serializing_if = "Option::is_none")]
    pub previous_uri: Option<String>,
    #[serde(rename = "lastUri", skip_serializing_if = "Option::is_none")]
    pub last_uri: Option<String>,
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
}

impl DevelopmentActivityListing {
    pub fn new() -> DevelopmentActivityListing {
        DevelopmentActivityListing {
            entities: None,
            page_size: None,
            page_number: None,
            total: None,
            first_uri: None,
            self_uri: None,
            next_uri: None,
            previous_uri: None,
            last_uri: None,
            page_count: None,
        }
    }
}


