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
pub struct Coretype {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A positive integer denoting the core type's version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The date the core type was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<crate::models::Schema>>,
    /// A boolean indicating if the core type's version is the current one in use by the system
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    /// An array of strings naming the fields of the core type subject to validation.  Validation constraints are specified by a schema author using the core type.
    #[serde(rename = "validationFields", skip_serializing_if = "Option::is_none")]
    pub validation_fields: Option<Vec<String>>,
    #[serde(rename = "validationLimits", skip_serializing_if = "Option::is_none")]
    pub validation_limits: Option<Box<crate::models::ValidationLimits>>,
    /// Specific to the \"tag\" core type, this is an array of strings naming the tag item fields of the core type subject to validation
    #[serde(rename = "itemValidationFields", skip_serializing_if = "Option::is_none")]
    pub item_validation_fields: Option<Vec<String>>,
    #[serde(rename = "itemValidationLimits", skip_serializing_if = "Option::is_none")]
    pub item_validation_limits: Option<Box<crate::models::ItemValidationLimits>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Coretype {
    pub fn new() -> Coretype {
        Coretype {
            id: None,
            name: None,
            version: None,
            date_created: None,
            schema: None,
            current: None,
            validation_fields: None,
            validation_limits: None,
            item_validation_fields: None,
            item_validation_limits: None,
            self_uri: None,
        }
    }
}

