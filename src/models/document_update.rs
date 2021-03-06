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
pub struct DocumentUpdate {
    #[serde(rename = "changeNumber", skip_serializing_if = "Option::is_none")]
    pub change_number: Option<i32>,
    /// The name of the document
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "addTags", skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<String>>,
    #[serde(rename = "removeTags", skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
    #[serde(rename = "addTagIds", skip_serializing_if = "Option::is_none")]
    pub add_tag_ids: Option<Vec<String>>,
    #[serde(rename = "removeTagIds", skip_serializing_if = "Option::is_none")]
    pub remove_tag_ids: Option<Vec<String>>,
    #[serde(rename = "updateAttributes", skip_serializing_if = "Option::is_none")]
    pub update_attributes: Option<Vec<crate::models::DocumentAttribute>>,
    #[serde(rename = "removeAttributes", skip_serializing_if = "Option::is_none")]
    pub remove_attributes: Option<Vec<String>>,
}

impl DocumentUpdate {
    pub fn new(name: String) -> DocumentUpdate {
        DocumentUpdate {
            change_number: None,
            name,
            read: None,
            add_tags: None,
            remove_tags: None,
            add_tag_ids: None,
            remove_tag_ids: None,
            update_attributes: None,
            remove_attributes: None,
        }
    }
}


