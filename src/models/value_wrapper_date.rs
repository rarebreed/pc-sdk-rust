/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ValueWrapperDate : An object to provide context to nullable fields in PATCH requests



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValueWrapperDate {
    /// The value for the associated field. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ValueWrapperDate {
    /// An object to provide context to nullable fields in PATCH requests
    pub fn new() -> ValueWrapperDate {
        ValueWrapperDate {
            value: None,
        }
    }
}


