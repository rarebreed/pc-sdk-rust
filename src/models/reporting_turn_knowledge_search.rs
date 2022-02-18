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
pub struct ReportingTurnKnowledgeSearch {
    /// The ID of this knowledge search.
    #[serde(rename = "searchId", skip_serializing_if = "Option::is_none")]
    pub search_id: Option<String>,
    /// The list of search documents captured during this reporting turn.
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<crate::models::ReportingTurnKnowledgeDocument>>,
    /// The search query that was used to search the Knowledge Base documents for a matching question.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl ReportingTurnKnowledgeSearch {
    pub fn new() -> ReportingTurnKnowledgeSearch {
        ReportingTurnKnowledgeSearch {
            search_id: None,
            documents: None,
            query: None,
        }
    }
}

