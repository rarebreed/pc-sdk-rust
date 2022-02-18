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
pub struct KnowledgeDocumentBulkRequest {
    /// Document type according to assigned template
    #[serde(rename = "type")]
    pub _type: Type,
    /// External Url to the document
    #[serde(rename = "externalUrl", skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(rename = "faq", skip_serializing_if = "Option::is_none")]
    pub faq: Option<Box<crate::models::DocumentFaq>>,
    /// Document categories
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<crate::models::DocumentCategoryInput>>,
    #[serde(rename = "article", skip_serializing_if = "Option::is_none")]
    pub article: Option<Box<crate::models::DocumentArticle>>,
    /// Identifier of document for update. Omit for create new Document.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl KnowledgeDocumentBulkRequest {
    pub fn new(_type: Type) -> KnowledgeDocumentBulkRequest {
        KnowledgeDocumentBulkRequest {
            _type,
            external_url: None,
            faq: None,
            categories: None,
            article: None,
            id: None,
        }
    }
}

/// Document type according to assigned template
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Faq")]
    Faq,
    #[serde(rename = "Article")]
    Article,
}

impl Default for Type {
    fn default() -> Type {
        Self::Faq
    }
}

