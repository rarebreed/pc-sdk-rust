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
pub struct DocumentArticle {
    /// The title of the Article.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<crate::models::ArticleContent>>,
    /// List of Alternative questions related to the title which helps in improving the likelihood of a match to user query.
    #[serde(rename = "alternatives", skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<String>>,
}

impl DocumentArticle {
    pub fn new(title: String) -> DocumentArticle {
        DocumentArticle {
            title,
            content: None,
            alternatives: None,
        }
    }
}


