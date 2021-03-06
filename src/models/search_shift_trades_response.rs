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
pub struct SearchShiftTradesResponse {
    /// The shift trades that match the search criteria
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<crate::models::SearchShiftTradeResponse>>,
}

impl SearchShiftTradesResponse {
    pub fn new() -> SearchShiftTradesResponse {
        SearchShiftTradesResponse {
            trades: None,
        }
    }
}


