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
pub struct AsyncIntradayResponse {
    /// The status of the operation
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The ID for the operation
    #[serde(rename = "operationId", skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::BuIntradayResponse>>,
}

impl AsyncIntradayResponse {
    pub fn new() -> AsyncIntradayResponse {
        AsyncIntradayResponse {
            status: None,
            operation_id: None,
            result: None,
        }
    }
}

/// The status of the operation
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "Error")]
    Error,
}

impl Default for Status {
    fn default() -> Status {
        Self::Processing
    }
}

