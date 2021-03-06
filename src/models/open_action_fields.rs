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
pub struct OpenActionFields {
    #[serde(rename = "openAction")]
    pub open_action: Box<crate::models::DomainEntityRef>,
    /// Custom fields defined in the schema referenced by the open action type selected.
    #[serde(rename = "configurationFields", skip_serializing_if = "Option::is_none")]
    pub configuration_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl OpenActionFields {
    pub fn new(open_action: crate::models::DomainEntityRef) -> OpenActionFields {
        OpenActionFields {
            open_action: Box::new(open_action),
            configuration_fields: None,
        }
    }
}


