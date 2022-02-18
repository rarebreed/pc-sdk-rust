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
pub struct AmazonLexRequest {
    /// AttributeName/AttributeValue pairs of User Defined Request Attributes to be sent to the amazon bot See - https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html#context-mgmt-request-attribs
    #[serde(rename = "requestAttributes", skip_serializing_if = "Option::is_none")]
    pub request_attributes: Option<::std::collections::HashMap<String, String>>,
    /// AttributeName/AttributeValue pairs of Session Attributes to be sent to the amazon bot. See - https://docs.aws.amazon.com/lex/latest/dg/context-mgmt.html#context-mgmt-session-attribs
    #[serde(rename = "sessionAttributes", skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<::std::collections::HashMap<String, String>>,
}

impl AmazonLexRequest {
    pub fn new() -> AmazonLexRequest {
        AmazonLexRequest {
            request_attributes: None,
            session_attributes: None,
        }
    }
}


