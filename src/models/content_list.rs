/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ContentList : List content object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContentList {
    /// A unique ID assigned to this rich message content.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of list this instance represents.
    #[serde(rename = "listType", skip_serializing_if = "Option::is_none")]
    pub list_type: Option<ListType>,
    /// Text to show in the title.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Text to show in the description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Label for Submit button.
    #[serde(rename = "submitLabel", skip_serializing_if = "Option::is_none")]
    pub submit_label: Option<String>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Box<crate::models::ContentActions>>,
    /// An array of component objects.
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<crate::models::ListItemComponent>>,
}

impl ContentList {
    /// List content object.
    pub fn new() -> ContentList {
        ContentList {
            id: None,
            list_type: None,
            title: None,
            description: None,
            submit_label: None,
            actions: None,
            components: None,
        }
    }
}

/// The type of list this instance represents.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListType {
    #[serde(rename = "Selection")]
    Selection,
    #[serde(rename = "Vertical")]
    Vertical,
}

impl Default for ListType {
    fn default() -> ListType {
        Self::Selection
    }
}

