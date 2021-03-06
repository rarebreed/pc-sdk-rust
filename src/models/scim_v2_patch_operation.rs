/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimV2PatchOperation : Defines a SCIM PATCH operation. The path and value follow very specific rules based on operation types. See section 3.5.2 \"Modifying with PATCH\" in RFC 7644 for details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimV2PatchOperation {
    /// The PATCH operation to perform.
    #[serde(rename = "op")]
    pub op: Op,
    /// The attribute path that describes the target of the operation. Required for a \"remove\" operation.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl ScimV2PatchOperation {
    /// Defines a SCIM PATCH operation. The path and value follow very specific rules based on operation types. See section 3.5.2 \"Modifying with PATCH\" in RFC 7644 for details.
    pub fn new(op: Op) -> ScimV2PatchOperation {
        ScimV2PatchOperation {
            op,
            path: None,
            value: None,
        }
    }
}

/// The PATCH operation to perform.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "remove")]
    Remove,
}

impl Default for Op {
    fn default() -> Op {
        Self::Add
    }
}

