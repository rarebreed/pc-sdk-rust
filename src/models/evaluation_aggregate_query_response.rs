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
pub struct EvaluationAggregateQueryResponse {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::EvaluationAggregateDataContainer>>,
}

impl EvaluationAggregateQueryResponse {
    pub fn new() -> EvaluationAggregateQueryResponse {
        EvaluationAggregateQueryResponse {
            results: None,
        }
    }
}


