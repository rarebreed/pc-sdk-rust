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
pub struct ApiUsageRow {
    /// Client Id associated with this query result
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Client Name associated with this query result
    #[serde(rename = "clientName", skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// Organization Id associated with this query result
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// User Id associated with this query result
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Template Uri associated with this query result
    #[serde(rename = "templateUri", skip_serializing_if = "Option::is_none")]
    pub template_uri: Option<String>,
    /// HTTP Method associated with this query result
    #[serde(rename = "httpMethod", skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// Number of requests resulting in a 2xx HTTP status code
    #[serde(rename = "status200", skip_serializing_if = "Option::is_none")]
    pub status200: Option<i64>,
    /// Number of requests resulting in a 3xx HTTP status code
    #[serde(rename = "status300", skip_serializing_if = "Option::is_none")]
    pub status300: Option<i64>,
    /// Number of requests resulting in a 4xx HTTP status code
    #[serde(rename = "status400", skip_serializing_if = "Option::is_none")]
    pub status400: Option<i64>,
    /// Number of requests resulting in a 5xx HTTP status code
    #[serde(rename = "status500", skip_serializing_if = "Option::is_none")]
    pub status500: Option<i64>,
    /// Number of requests resulting in a 429 HTTP status code, this is a subset of the count returned with status400
    #[serde(rename = "status429", skip_serializing_if = "Option::is_none")]
    pub status429: Option<i64>,
    /// Total number of requests
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<i64>,
    /// Date of requests, based on granularity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl ApiUsageRow {
    pub fn new() -> ApiUsageRow {
        ApiUsageRow {
            client_id: None,
            client_name: None,
            organization_id: None,
            user_id: None,
            template_uri: None,
            http_method: None,
            status200: None,
            status300: None,
            status400: None,
            status500: None,
            status429: None,
            requests: None,
            date: None,
        }
    }
}


