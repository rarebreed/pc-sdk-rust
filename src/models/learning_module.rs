/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// LearningModule : Learning module response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LearningModule {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of learning module
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::UserReference>>,
    /// The date/time learning module was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::UserReference>>,
    /// The date/time learning module was modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// The version of published learning module
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The external ID of the learning module
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The source of the learning module
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "rule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<Box<crate::models::LearningModuleRule>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// If true, learning module is archived
    #[serde(rename = "isArchived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// If true, learning module is published
    #[serde(rename = "isPublished", skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
    /// The description of learning module
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The completion time of learning module in days
    #[serde(rename = "completionTimeInDays")]
    pub completion_time_in_days: i32,
    /// The type for the learning module
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The list of inform steps in a learning module
    #[serde(rename = "informSteps", skip_serializing_if = "Option::is_none")]
    pub inform_steps: Option<Vec<crate::models::LearningModuleInformStep>>,
    #[serde(rename = "assessmentForm", skip_serializing_if = "Option::is_none")]
    pub assessment_form: Option<Box<crate::models::AssessmentForm>>,
    #[serde(rename = "summaryData", skip_serializing_if = "Option::is_none")]
    pub summary_data: Option<Box<crate::models::LearningModuleSummary>>,
}

impl LearningModule {
    /// Learning module response
    pub fn new(name: String, completion_time_in_days: i32) -> LearningModule {
        LearningModule {
            id: None,
            name,
            created_by: None,
            date_created: None,
            modified_by: None,
            date_modified: None,
            version: None,
            external_id: None,
            source: None,
            rule: None,
            self_uri: None,
            is_archived: None,
            is_published: None,
            description: None,
            completion_time_in_days,
            _type: None,
            inform_steps: None,
            assessment_form: None,
            summary_data: None,
        }
    }
}

/// The source of the learning module
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "UserCreated")]
    UserCreated,
    #[serde(rename = "GenesysBeyond")]
    GenesysBeyond,
}

impl Default for Source {
    fn default() -> Source {
        Self::UserCreated
    }
}
/// The type for the learning module
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Informational")]
    Informational,
    #[serde(rename = "AssessedContent")]
    AssessedContent,
    #[serde(rename = "Assessment")]
    Assessment,
}

impl Default for Type {
    fn default() -> Type {
        Self::Informational
    }
}

