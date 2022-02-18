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
pub struct PhoneNumberColumn {
    #[serde(rename = "columnName", skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl PhoneNumberColumn {
    pub fn new() -> PhoneNumberColumn {
        PhoneNumberColumn {
            column_name: None,
            _type: None,
        }
    }
}


