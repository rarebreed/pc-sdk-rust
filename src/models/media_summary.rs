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
pub struct MediaSummary {
    #[serde(rename = "contactCenter", skip_serializing_if = "Option::is_none")]
    pub contact_center: Option<Box<crate::models::MediaSummaryDetail>>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<crate::models::MediaSummaryDetail>>,
}

impl MediaSummary {
    pub fn new() -> MediaSummary {
        MediaSummary {
            contact_center: None,
            enterprise: None,
        }
    }
}


