/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// DataTable : Contains a metadata representation for a JSON schema stored in DataTables along with an optional field for the schema itself



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataTable {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::WritableDivision>>,
    /// The description from the JSON schema (equates to the Description field on the JSON schema.)
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<crate::models::JsonSchemaDocument>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl DataTable {
    /// Contains a metadata representation for a JSON schema stored in DataTables along with an optional field for the schema itself
    pub fn new() -> DataTable {
        DataTable {
            id: None,
            name: None,
            division: None,
            description: None,
            schema: None,
            self_uri: None,
        }
    }
}


