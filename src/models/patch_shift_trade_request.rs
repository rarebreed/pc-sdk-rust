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
pub struct PatchShiftTradeRequest {
    #[serde(rename = "receivingUserId", skip_serializing_if = "Option::is_none")]
    pub receiving_user_id: Option<Box<crate::models::ValueWrapperString>>,
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<Box<crate::models::ValueWrapperDate>>,
    #[serde(rename = "acceptableIntervals", skip_serializing_if = "Option::is_none")]
    pub acceptable_intervals: Option<Box<crate::models::ListWrapperInterval>>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::WfmVersionedEntityMetadata>,
}

impl PatchShiftTradeRequest {
    pub fn new(metadata: crate::models::WfmVersionedEntityMetadata) -> PatchShiftTradeRequest {
        PatchShiftTradeRequest {
            receiving_user_id: None,
            expiration: None,
            acceptable_intervals: None,
            metadata: Box::new(metadata),
        }
    }
}

