/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// WebMessagingButtonResponse : Button response object representing the click of a structured message button, such as a quick reply.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebMessagingButtonResponse {
    /// An ID assigned to the button response (Deprecated).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Describes the button that resulted in the Button Response.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The response text from the button click.
    #[serde(rename = "text")]
    pub text: String,
    /// The response payload associated with the clicked button.
    #[serde(rename = "payload")]
    pub payload: String,
}

impl WebMessagingButtonResponse {
    /// Button response object representing the click of a structured message button, such as a quick reply.
    pub fn new(text: String, payload: String) -> WebMessagingButtonResponse {
        WebMessagingButtonResponse {
            id: None,
            _type: None,
            text,
            payload,
        }
    }
}

/// Describes the button that resulted in the Button Response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Button")]
    Button,
    #[serde(rename = "QuickReply")]
    QuickReply,
}

impl Default for Type {
    fn default() -> Type {
        Self::Button
    }
}
