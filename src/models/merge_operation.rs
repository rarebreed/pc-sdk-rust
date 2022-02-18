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
pub struct MergeOperation {
    #[serde(rename = "sourceContact", skip_serializing_if = "Option::is_none")]
    pub source_contact: Option<Box<crate::models::AddressableEntityRef>>,
    #[serde(rename = "targetContact", skip_serializing_if = "Option::is_none")]
    pub target_contact: Option<Box<crate::models::AddressableEntityRef>>,
    #[serde(rename = "resultingContact", skip_serializing_if = "Option::is_none")]
    pub resulting_contact: Option<Box<crate::models::AddressableEntityRef>>,
}

impl MergeOperation {
    pub fn new() -> MergeOperation {
        MergeOperation {
            source_contact: None,
            target_contact: None,
            resulting_contact: None,
        }
    }
}

