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
pub struct Utilization {
    /// Map of media type to utilization settings.  Valid media types include call, callback, chat, email, and message.
    #[serde(rename = "utilization", skip_serializing_if = "Option::is_none")]
    pub utilization: Option<::std::collections::HashMap<String, crate::models::MediaUtilization>>,
}

impl Utilization {
    pub fn new() -> Utilization {
        Utilization {
            utilization: None,
        }
    }
}

