/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimConfigResourceType : Defines a SCIM resource.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimConfigResourceType {
    /// The ID of the SCIM resource. Set by the service provider. \"caseExact\" is set to \"true\". \"mutability\" is set to \"readOnly\". \"returned\" is set to \"always\".
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The list of supported schemas.
    #[serde(rename = "schemas", skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    /// The name of the resource type.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the resource type.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URI of the primary or base schema for the resource type.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// The list of schema extensions for the resource type.
    #[serde(rename = "schemaExtensions", skip_serializing_if = "Option::is_none")]
    pub schema_extensions: Option<Vec<crate::models::ScimConfigResourceTypeSchemaExtension>>,
    /// The HTTP-addressable endpoint of the resource type. Appears after the base URL.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ScimMetadata>>,
}

impl ScimConfigResourceType {
    /// Defines a SCIM resource.
    pub fn new() -> ScimConfigResourceType {
        ScimConfigResourceType {
            id: None,
            schemas: None,
            name: None,
            description: None,
            schema: None,
            schema_extensions: None,
            endpoint: None,
            meta: None,
        }
    }
}


