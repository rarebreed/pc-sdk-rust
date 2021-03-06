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
pub struct OAuthLastTokenIssued {
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateIssued", skip_serializing_if = "Option::is_none")]
    pub date_issued: Option<String>,
}

impl OAuthLastTokenIssued {
    pub fn new() -> OAuthLastTokenIssued {
        OAuthLastTokenIssued {
            date_issued: None,
        }
    }
}


