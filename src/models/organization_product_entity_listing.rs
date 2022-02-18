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
pub struct OrganizationProductEntityListing {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::DomainOrganizationProduct>>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "pageNumber", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
}

impl OrganizationProductEntityListing {
    pub fn new() -> OrganizationProductEntityListing {
        OrganizationProductEntityListing {
            entities: None,
            page_size: None,
            page_number: None,
            total: None,
            page_count: None,
        }
    }
}


