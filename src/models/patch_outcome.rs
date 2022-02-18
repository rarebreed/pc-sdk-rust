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
pub struct PatchOutcome {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether or not the outcome is active.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// The display name of the outcome.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// The version of the outcome.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// A description of the outcome.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether or not the outcome is positive.
    #[serde(rename = "isPositive", skip_serializing_if = "Option::is_none")]
    pub is_positive: Option<bool>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<crate::models::Context>>,
    #[serde(rename = "journey", skip_serializing_if = "Option::is_none")]
    pub journey: Option<Box<crate::models::Journey>>,
    #[serde(rename = "associatedValueField", skip_serializing_if = "Option::is_none")]
    pub associated_value_field: Option<Box<crate::models::AssociatedValueField>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    /// Timestamp indicating when the outcome was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// Timestamp indicating when the outcome was last updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "modifiedDate", skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<String>,
}

impl PatchOutcome {
    pub fn new(display_name: String) -> PatchOutcome {
        PatchOutcome {
            id: None,
            is_active: None,
            display_name,
            version: None,
            description: None,
            is_positive: None,
            context: None,
            journey: None,
            associated_value_field: None,
            self_uri: None,
            created_date: None,
            modified_date: None,
        }
    }
}


