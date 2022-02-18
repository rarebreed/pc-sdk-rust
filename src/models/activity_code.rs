/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ActivityCode : Activity code data



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActivityCode {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// The name of the activity code. Default activity codes will be created with an empty name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this activity code is active or has been deleted
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Whether this is a default activity code
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The activity code's category.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// The default length of the activity in minutes
    #[serde(rename = "lengthInMinutes", skip_serializing_if = "Option::is_none")]
    pub length_in_minutes: Option<i32>,
    /// Whether an agent is paid while performing this activity
    #[serde(rename = "countsAsPaidTime", skip_serializing_if = "Option::is_none")]
    pub counts_as_paid_time: Option<bool>,
    /// Indicates whether or not the activity should be counted as contiguous work time for calculating daily constraints
    #[serde(rename = "countsAsWorkTime", skip_serializing_if = "Option::is_none")]
    pub counts_as_work_time: Option<bool>,
    /// Whether an agent can select this activity code when creating or editing a time off request. Null if the activity's category is not time off.
    #[serde(rename = "agentTimeOffSelectable", skip_serializing_if = "Option::is_none")]
    pub agent_time_off_selectable: Option<bool>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::WfmVersionedEntityMetadata>,
}

impl ActivityCode {
    /// Activity code data
    pub fn new(metadata: crate::models::WfmVersionedEntityMetadata) -> ActivityCode {
        ActivityCode {
            id: None,
            self_uri: None,
            name: None,
            is_active: None,
            is_default: None,
            category: None,
            length_in_minutes: None,
            counts_as_paid_time: None,
            counts_as_work_time: None,
            agent_time_off_selectable: None,
            metadata: Box::new(metadata),
        }
    }
}

/// The activity code's category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "OnQueueWork")]
    OnQueueWork,
    #[serde(rename = "Break")]
    _Break,
    #[serde(rename = "Meal")]
    Meal,
    #[serde(rename = "Meeting")]
    Meeting,
    #[serde(rename = "OffQueueWork")]
    OffQueueWork,
    #[serde(rename = "TimeOff")]
    TimeOff,
    #[serde(rename = "Training")]
    Training,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "Unscheduled")]
    Unscheduled,
}

impl Default for Category {
    fn default() -> Category {
        Self::OnQueueWork
    }
}

