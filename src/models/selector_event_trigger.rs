/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// SelectorEventTrigger : Details about a selector event trigger



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SelectorEventTrigger {
    /// Element that triggers event.
    #[serde(rename = "selector")]
    pub selector: String,
    /// Name of event triggered when element matching selector is interacted with.
    #[serde(rename = "eventName")]
    pub event_name: String,
}

impl SelectorEventTrigger {
    /// Details about a selector event trigger
    pub fn new(selector: String, event_name: String) -> SelectorEventTrigger {
        SelectorEventTrigger {
            selector,
            event_name,
        }
    }
}


