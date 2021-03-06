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
pub struct BulkUpdateShiftTradeStateResult {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::BulkUpdateShiftTradeStateResultItem>>,
}

impl BulkUpdateShiftTradeStateResult {
    pub fn new() -> BulkUpdateShiftTradeStateResult {
        BulkUpdateShiftTradeStateResult {
            entities: None,
        }
    }
}


