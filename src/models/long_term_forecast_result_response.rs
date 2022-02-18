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
pub struct LongTermForecastResultResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::LongTermForecastResult>>,
    /// The download url to fetch the result.  Only populated if the result is too large to pass through the api directly
    #[serde(rename = "downloadUrl", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

impl LongTermForecastResultResponse {
    pub fn new() -> LongTermForecastResultResponse {
        LongTermForecastResultResponse {
            result: None,
            download_url: None,
        }
    }
}


