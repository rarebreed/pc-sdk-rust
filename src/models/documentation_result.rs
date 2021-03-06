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
pub struct DocumentationResult {
    /// The globally unique identifier for the object.
    #[serde(rename = "id")]
    pub id: i32,
    /// The category of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i32>>,
    /// The description of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The text or html content for the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The excerpt of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "excerpt", skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    /// URL link for the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The modified date for the documentation entity. Will be returned in responses for certain entities. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    /// The name of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The service of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<i32>>,
    /// The slug of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// The title of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The search type. Will be returned in responses for certain entities.
    #[serde(rename = "get_type", skip_serializing_if = "Option::is_none")]
    pub get_type: Option<String>,
    /// The facet feature of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "facet_feature", skip_serializing_if = "Option::is_none")]
    pub facet_feature: Option<Vec<i32>>,
    /// The facet role of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "facet_role", skip_serializing_if = "Option::is_none")]
    pub facet_role: Option<Vec<i32>>,
    /// The facet service of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "facet_service", skip_serializing_if = "Option::is_none")]
    pub facet_service: Option<Vec<i32>>,
    /// The faq categories of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "faq_categories", skip_serializing_if = "Option::is_none")]
    pub faq_categories: Option<Vec<i32>>,
    /// The releasenote category of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "releasenote_category", skip_serializing_if = "Option::is_none")]
    pub releasenote_category: Option<Vec<i32>>,
    /// The releasenote tag of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "releasenote_tag", skip_serializing_if = "Option::is_none")]
    pub releasenote_tag: Option<Vec<i32>>,
    /// The service area of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "service-area", skip_serializing_if = "Option::is_none")]
    pub service_area: Option<Vec<i32>>,
    /// The video categories of the documentation entity. Will be returned in responses for certain entities.
    #[serde(rename = "video_categories", skip_serializing_if = "Option::is_none")]
    pub video_categories: Option<Vec<i32>>,
}

impl DocumentationResult {
    pub fn new(id: i32) -> DocumentationResult {
        DocumentationResult {
            id,
            categories: None,
            description: None,
            content: None,
            excerpt: None,
            link: None,
            modified: None,
            name: None,
            service: None,
            slug: None,
            title: None,
            get_type: None,
            facet_feature: None,
            facet_role: None,
            facet_service: None,
            faq_categories: None,
            releasenote_category: None,
            releasenote_tag: None,
            service_area: None,
            video_categories: None,
        }
    }
}


