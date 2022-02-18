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
pub struct SchemaQuantityLimits {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The minimum number of schema field name characters allowed.
    #[serde(rename = "minFieldNameCharacters", skip_serializing_if = "Option::is_none")]
    pub min_field_name_characters: Option<i32>,
    /// The maximum number of schema field name characters allowed.
    #[serde(rename = "maxFieldNameCharacters", skip_serializing_if = "Option::is_none")]
    pub max_field_name_characters: Option<i32>,
    /// The minimum number of schema field description characters allowed.
    #[serde(rename = "minFieldDescriptionCharacters", skip_serializing_if = "Option::is_none")]
    pub min_field_description_characters: Option<i32>,
    /// The maximum number of schema field description characters allowed.
    #[serde(rename = "maxFieldDescriptionCharacters", skip_serializing_if = "Option::is_none")]
    pub max_field_description_characters: Option<i32>,
    /// The minimum number of schema name characters allowed.
    #[serde(rename = "minSchemaNameCharacters", skip_serializing_if = "Option::is_none")]
    pub min_schema_name_characters: Option<i32>,
    /// The maximum number of schema name characters allowed.
    #[serde(rename = "maxSchemaNameCharacters", skip_serializing_if = "Option::is_none")]
    pub max_schema_name_characters: Option<i32>,
    /// The minimum number of schema description characters allowed.
    #[serde(rename = "minSchemaDescriptionCharacters", skip_serializing_if = "Option::is_none")]
    pub min_schema_description_characters: Option<i32>,
    /// The maximum number of schema description characters allowed.
    #[serde(rename = "maxSchemaDescriptionCharacters", skip_serializing_if = "Option::is_none")]
    pub max_schema_description_characters: Option<i32>,
    /// The maximum number of schema allowed per org.
    #[serde(rename = "maxNumberOfSchemasPerOrg", skip_serializing_if = "Option::is_none")]
    pub max_number_of_schemas_per_org: Option<i32>,
    /// The maximum number of schema fields allowed per schema.
    #[serde(rename = "maxNumberOfFieldsPerSchema", skip_serializing_if = "Option::is_none")]
    pub max_number_of_fields_per_schema: Option<i32>,
    /// The maximum number of schema fields allowed per organization across all of their schemas.
    #[serde(rename = "maxNumberOfFieldsPerOrg", skip_serializing_if = "Option::is_none")]
    pub max_number_of_fields_per_org: Option<i32>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl SchemaQuantityLimits {
    pub fn new() -> SchemaQuantityLimits {
        SchemaQuantityLimits {
            id: None,
            name: None,
            min_field_name_characters: None,
            max_field_name_characters: None,
            min_field_description_characters: None,
            max_field_description_characters: None,
            min_schema_name_characters: None,
            max_schema_name_characters: None,
            min_schema_description_characters: None,
            max_schema_description_characters: None,
            max_number_of_schemas_per_org: None,
            max_number_of_fields_per_schema: None,
            max_number_of_fields_per_org: None,
            self_uri: None,
        }
    }
}


