/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimV2SchemaAttribute : A complex type that defines service provider attributes or subattributes and their qualities.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimV2SchemaAttribute {
    /// The name of the attribute.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The data type of the attribute.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The list of subattributes for an attribute of the type \"complex\". Uses the same schema as \"attributes\".
    #[serde(rename = "subAttributes", skip_serializing_if = "Option::is_none")]
    pub sub_attributes: Option<Vec<crate::models::ScimV2SchemaAttribute>>,
    /// Indicates whether an attribute contains multiple values.
    #[serde(rename = "multiValued", skip_serializing_if = "Option::is_none")]
    pub multi_valued: Option<bool>,
    /// The description of the attribute.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates whether an attribute is required.
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// The list of standard values that service providers may use. Service providers may ignore unsupported values.
    #[serde(rename = "canonicalValues", skip_serializing_if = "Option::is_none")]
    pub canonical_values: Option<Vec<String>>,
    /// Indicates whether a string attribute is case-sensitive. If set to \"true\", the server preserves case sensitivity. If set to \"false\", the server may change the case. The server also uses case sensitivity when evaluating filters. See section 3.4.2.2 \"Filtering\" in RFC 7644 for details.
    #[serde(rename = "caseExact", skip_serializing_if = "Option::is_none")]
    pub case_exact: Option<bool>,
    /// The circumstances under which an attribute can be defined or redefined. The default is \"readWrite\".
    #[serde(rename = "mutability", skip_serializing_if = "Option::is_none")]
    pub mutability: Option<Mutability>,
    /// The circumstances under which an attribute and its values are returned in response to a GET, PUT, POST, or PATCH request.
    #[serde(rename = "returned", skip_serializing_if = "Option::is_none")]
    pub returned: Option<Returned>,
    /// The method by which the service provider enforces the uniqueness of an attribute value. A server can reject a value by returning the HTTP response code 400 (Bad Request). A client can enforce uniqueness to a greater degree than the server provider enforces. For example, a client could make a value unique even though the server has \"uniqueness\" set to \"none\".
    #[serde(rename = "uniqueness", skip_serializing_if = "Option::is_none")]
    pub uniqueness: Option<Uniqueness>,
    /// The list of SCIM resource types that may be referenced. Only applies when \"type\" is set to \"reference\".
    #[serde(rename = "referenceTypes", skip_serializing_if = "Option::is_none")]
    pub reference_types: Option<Vec<ReferenceTypes>>,
}

impl ScimV2SchemaAttribute {
    /// A complex type that defines service provider attributes or subattributes and their qualities.
    pub fn new() -> ScimV2SchemaAttribute {
        ScimV2SchemaAttribute {
            name: None,
            _type: None,
            sub_attributes: None,
            multi_valued: None,
            description: None,
            required: None,
            canonical_values: None,
            case_exact: None,
            mutability: None,
            returned: None,
            uniqueness: None,
            reference_types: None,
        }
    }
}

/// The data type of the attribute.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "dateTime")]
    DateTime,
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "complex")]
    Complex,
}

impl Default for Type {
    fn default() -> Type {
        Self::String
    }
}
/// The circumstances under which an attribute can be defined or redefined. The default is \"readWrite\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mutability {
    #[serde(rename = "readWrite")]
    ReadWrite,
    #[serde(rename = "readOnly")]
    ReadOnly,
    #[serde(rename = "immutable")]
    Immutable,
    #[serde(rename = "writeOnly")]
    WriteOnly,
}

impl Default for Mutability {
    fn default() -> Mutability {
        Self::ReadWrite
    }
}
/// The circumstances under which an attribute and its values are returned in response to a GET, PUT, POST, or PATCH request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Returned {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "request")]
    Request,
}

impl Default for Returned {
    fn default() -> Returned {
        Self::Always
    }
}
/// The method by which the service provider enforces the uniqueness of an attribute value. A server can reject a value by returning the HTTP response code 400 (Bad Request). A client can enforce uniqueness to a greater degree than the server provider enforces. For example, a client could make a value unique even though the server has \"uniqueness\" set to \"none\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Uniqueness {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "global")]
    Global,
}

impl Default for Uniqueness {
    fn default() -> Uniqueness {
        Self::None
    }
}
/// The list of SCIM resource types that may be referenced. Only applies when \"type\" is set to \"reference\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReferenceTypes {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Group")]
    Group,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "uri")]
    Uri,
}

impl Default for ReferenceTypes {
    fn default() -> ReferenceTypes {
        Self::User
    }
}

