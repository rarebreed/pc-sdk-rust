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
pub struct ProgramMappings {
    #[serde(rename = "program", skip_serializing_if = "Option::is_none")]
    pub program: Option<Box<crate::models::BaseProgramEntity>>,
    #[serde(rename = "queues", skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<crate::models::AddressableEntityRef>>,
    #[serde(rename = "flows", skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<crate::models::AddressableEntityRef>>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::AddressableEntityRef>>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
}

impl ProgramMappings {
    pub fn new() -> ProgramMappings {
        ProgramMappings {
            program: None,
            queues: None,
            flows: None,
            modified_by: None,
            date_modified: None,
        }
    }
}


