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
pub struct ArticleContentBody {
    /// Presigned URL to retrieve the document content.
    #[serde(rename = "locationUrl", skip_serializing_if = "Option::is_none")]
    pub location_url: Option<String>,
}

impl ArticleContentBody {
    pub fn new() -> ArticleContentBody {
        ArticleContentBody {
            location_url: None,
        }
    }
}


