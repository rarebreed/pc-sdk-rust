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
pub struct FlowVersion {
    /// The flow version identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "commitVersion", skip_serializing_if = "Option::is_none")]
    pub commit_version: Option<String>,
    #[serde(rename = "configurationVersion", skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::User>>,
    #[serde(rename = "createdByClient", skip_serializing_if = "Option::is_none")]
    pub created_by_client: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "configurationUri", skip_serializing_if = "Option::is_none")]
    pub configuration_uri: Option<String>,
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<i64>,
    #[serde(rename = "generationId", skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(rename = "publishResultUri", skip_serializing_if = "Option::is_none")]
    pub publish_result_uri: Option<String>,
    #[serde(rename = "inputSchema", skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Box<crate::models::JsonSchemaDocument>>,
    #[serde(rename = "outputSchema", skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Box<crate::models::JsonSchemaDocument>>,
    #[serde(rename = "nluInfo", skip_serializing_if = "Option::is_none")]
    pub nlu_info: Option<Box<crate::models::NluInfo>>,
    /// List of supported languages for this version of the flow
    #[serde(rename = "supportedLanguages", skip_serializing_if = "Option::is_none")]
    pub supported_languages: Option<Vec<crate::models::SupportedLanguage>>,
    /// Compatible flow types designate which flow types are allowed to embed a flow???s configuration within their own flow configuration.  Currently the only flows that can be embedded are Common Module flows and the embedding flow can invoke them using the Call Common Module action.
    #[serde(rename = "compatibleFlowTypes", skip_serializing_if = "Option::is_none")]
    pub compatible_flow_types: Option<Vec<CompatibleFlowTypes>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl FlowVersion {
    pub fn new() -> FlowVersion {
        FlowVersion {
            id: None,
            name: None,
            commit_version: None,
            configuration_version: None,
            _type: None,
            secure: None,
            debug: None,
            created_by: None,
            created_by_client: None,
            configuration_uri: None,
            date_created: None,
            generation_id: None,
            publish_result_uri: None,
            input_schema: None,
            output_schema: None,
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
    #[serde(rename = "PUBLISH")]
    PUBLISH,
    #[serde(rename = "CHECKIN")]
    CHECKIN,
    #[serde(rename = "SAVE")]
    SAVE,
}

impl Default for Type {
    fn default() -> Type {
        Self::PUBLISH
    }
}
/// Compatible flow types designate which flow types are allowed to embed a flow???s configuration within their own flow configuration.  Currently the only flows that can be embedded are Common Module flows and the embedding flow can invoke them using the Call Common Module action.
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

