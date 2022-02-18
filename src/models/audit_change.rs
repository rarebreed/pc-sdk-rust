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
pub struct AuditChange {
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<crate::models::AuditEntityReference>>,
    #[serde(rename = "oldValues", skip_serializing_if = "Option::is_none")]
    pub old_values: Option<Vec<String>>,
    #[serde(rename = "newValues", skip_serializing_if = "Option::is_none")]
    pub new_values: Option<Vec<String>>,
}

impl AuditChange {
    pub fn new() -> AuditChange {
        AuditChange {
            property: None,
            entity: None,
            old_values: None,
            new_values: None,
        }
    }
}

