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
pub struct Flow {
    /// The flow identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The flow name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::WritableDivision>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "lockedUser", skip_serializing_if = "Option::is_none")]
    pub locked_user: Option<Box<crate::models::User>>,
    #[serde(rename = "lockedClient", skip_serializing_if = "Option::is_none")]
    pub locked_client: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "publishedVersion", skip_serializing_if = "Option::is_none")]
    pub published_version: Option<Box<crate::models::FlowVersion>>,
    #[serde(rename = "savedVersion", skip_serializing_if = "Option::is_none")]
    pub saved_version: Option<Box<crate::models::FlowVersion>>,
    /// json schema describing the inputs for the flow
    #[serde(rename = "inputSchema", skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<serde_json::Value>,
    /// json schema describing the outputs for the flow
    #[serde(rename = "outputSchema", skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
    #[serde(rename = "checkedInVersion", skip_serializing_if = "Option::is_none")]
    pub checked_in_version: Option<Box<crate::models::FlowVersion>>,
    #[serde(rename = "debugVersion", skip_serializing_if = "Option::is_none")]
    pub debug_version: Option<Box<crate::models::FlowVersion>>,
    #[serde(rename = "publishedBy", skip_serializing_if = "Option::is_none")]
    pub published_by: Option<Box<crate::models::User>>,
    #[serde(rename = "currentOperation", skip_serializing_if = "Option::is_none")]
    pub current_operation: Option<Box<crate::models::Operation>>,
    #[serde(rename = "nluInfo", skip_serializing_if = "Option::is_none")]
    pub nlu_info: Option<Box<crate::models::NluInfo>>,
    /// List of supported languages for the published version of the flow.
    #[serde(rename = "supportedLanguages", skip_serializing_if = "Option::is_none")]
    pub supported_languages: Option<Vec<crate::models::SupportedLanguage>>,
    /// Compatible flow types designate which flow types are allowed to embed a flow’s configuration within their own flow configuration.  Currently the only flows that can be embedded are Common Module flows and the embedding flow can invoke them using the Call Common Module action.
    #[serde(rename = "compatibleFlowTypes", skip_serializing_if = "Option::is_none")]
    pub compatible_flow_types: Option<Vec<CompatibleFlowTypes>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Flow {
    pub fn new(name: String) -> Flow {
        Flow {
            id: None,
            name,
            division: None,
            description: None,
            _type: None,
            locked_user: None,
            locked_client: None,
            active: None,
            system: None,
            deleted: None,
            published_version: None,
            saved_version: None,
            input_schema: None,
            output_schema: None,
            checked_in_version: None,
            debug_version: None,
            published_by: None,
            current_operation: None,
            nlu_info: None,
            supported_languages: None,
            compatible_flow_types: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BOT")]
    BOT,
    #[serde(rename = "COMMONMODULE")]
    COMMONMODULE,
    #[serde(rename = "INBOUNDCALL")]
    INBOUNDCALL,
    #[serde(rename = "INBOUNDCHAT")]
    INBOUNDCHAT,
    #[serde(rename = "INBOUNDEMAIL")]
    INBOUNDEMAIL,
    #[serde(rename = "INBOUNDSHORTMESSAGE")]
    INBOUNDSHORTMESSAGE,
    #[serde(rename = "INQUEUECALL")]
    INQUEUECALL,
    #[serde(rename = "INQUEUEEMAIL")]
    INQUEUEEMAIL,
    #[serde(rename = "INQUEUESHORTMESSAGE")]
    INQUEUESHORTMESSAGE,
    #[serde(rename = "OUTBOUNDCALL")]
    OUTBOUNDCALL,
    #[serde(rename = "SECURECALL")]
    SECURECALL,
    #[serde(rename = "SPEECH")]
    SPEECH,
    #[serde(rename = "SURVEYINVITE")]
    SURVEYINVITE,
    #[serde(rename = "VOICEMAIL")]
    VOICEMAIL,
    #[serde(rename = "WORKFLOW")]
    WORKFLOW,
    #[serde(rename = "WORKITEM")]
    WORKITEM,
}

impl Default for Type {
    fn default() -> Type {
        Self::BOT
    }
}
/// Compatible flow types designate which flow types are allowed to embed a flow’s configuration within their own flow configuration.  Currently the only flows that can be embedded are Common Module flows and the embedding flow can invoke them using the Call Common Module action.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompatibleFlowTypes {
    #[serde(rename = "BOT")]
    BOT,
    #[serde(rename = "COMMONMODULE")]
    COMMONMODULE,
    #[serde(rename = "INBOUNDCALL")]
    INBOUNDCALL,
    #[serde(rename = "INBOUNDCHAT")]
    INBOUNDCHAT,
    #[serde(rename = "INBOUNDEMAIL")]
    INBOUNDEMAIL,
    #[serde(rename = "INBOUNDSHORTMESSAGE")]
    INBOUNDSHORTMESSAGE,
    #[serde(rename = "INQUEUECALL")]
    INQUEUECALL,
    #[serde(rename = "INQUEUEEMAIL")]
    INQUEUEEMAIL,
    #[serde(rename = "INQUEUESHORTMESSAGE")]
    INQUEUESHORTMESSAGE,
    #[serde(rename = "OUTBOUNDCALL")]
    OUTBOUNDCALL,
    #[serde(rename = "SECURECALL")]
    SECURECALL,
    #[serde(rename = "SPEECH")]
    SPEECH,
    #[serde(rename = "SURVEYINVITE")]
    SURVEYINVITE,
    #[serde(rename = "VOICEMAIL")]
    VOICEMAIL,
    #[serde(rename = "WORKFLOW")]
    WORKFLOW,
    #[serde(rename = "WORKITEM")]
    WORKITEM,
}

impl Default for CompatibleFlowTypes {
    fn default() -> CompatibleFlowTypes {
        Self::BOT
    }
}

