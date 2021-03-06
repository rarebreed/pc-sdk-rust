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
pub struct DependencyObject {
    /// The dependency identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<bool>,
    #[serde(rename = "stateUnknown", skip_serializing_if = "Option::is_none")]
    pub state_unknown: Option<bool>,
    #[serde(rename = "consumedResources", skip_serializing_if = "Option::is_none")]
    pub consumed_resources: Option<Vec<crate::models::Dependency>>,
    #[serde(rename = "consumingResources", skip_serializing_if = "Option::is_none")]
    pub consuming_resources: Option<Vec<crate::models::Dependency>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl DependencyObject {
    pub fn new() -> DependencyObject {
        DependencyObject {
            id: None,
            name: None,
            version: None,
            _type: None,
            deleted: None,
            updated: None,
            state_unknown: None,
            consumed_resources: None,
            consuming_resources: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ACDLANGUAGE")]
    ACDLANGUAGE,
    #[serde(rename = "ACDSKILL")]
    ACDSKILL,
    #[serde(rename = "ACDWRAPUPCODE")]
    ACDWRAPUPCODE,
    #[serde(rename = "BOTCONNECTORBOT")]
    BOTCONNECTORBOT,
    #[serde(rename = "BOTCONNECTORINTEGRATION")]
    BOTCONNECTORINTEGRATION,
    #[serde(rename = "BOTFLOW")]
    BOTFLOW,
    #[serde(rename = "BRIDGEACTION")]
    BRIDGEACTION,
    #[serde(rename = "COMMONMODULEFLOW")]
    COMMONMODULEFLOW,
    #[serde(rename = "COMPOSERSCRIPT")]
    COMPOSERSCRIPT,
    #[serde(rename = "CONTACTLIST")]
    CONTACTLIST,
    #[serde(rename = "DATAACTION")]
    DATAACTION,
    #[serde(rename = "DATATABLE")]
    DATATABLE,
    #[serde(rename = "DIALOGENGINEBOT")]
    DIALOGENGINEBOT,
    #[serde(rename = "DIALOGENGINEBOTVERSION")]
    DIALOGENGINEBOTVERSION,
    #[serde(rename = "DIALOGFLOWAGENT")]
    DIALOGFLOWAGENT,
    #[serde(rename = "DIALOGFLOWCXAGENT")]
    DIALOGFLOWCXAGENT,
    #[serde(rename = "EMAILROUTE")]
    EMAILROUTE,
    #[serde(rename = "EMERGENCYGROUP")]
    EMERGENCYGROUP,
    #[serde(rename = "FLOWACTION")]
    FLOWACTION,
    #[serde(rename = "FLOWDATATYPE")]
    FLOWDATATYPE,
    #[serde(rename = "FLOWMILESTONE")]
    FLOWMILESTONE,
    #[serde(rename = "FLOWOUTCOME")]
    FLOWOUTCOME,
    #[serde(rename = "GROUP")]
    GROUP,
    #[serde(rename = "INBOUNDCALLFLOW")]
    INBOUNDCALLFLOW,
    #[serde(rename = "INBOUNDCHATFLOW")]
    INBOUNDCHATFLOW,
    #[serde(rename = "INBOUNDEMAILFLOW")]
    INBOUNDEMAILFLOW,
    #[serde(rename = "INBOUNDSHORTMESSAGEFLOW")]
    INBOUNDSHORTMESSAGEFLOW,
    #[serde(rename = "INQUEUECALLFLOW")]
    INQUEUECALLFLOW,
    #[serde(rename = "INQUEUEEMAILFLOW")]
    INQUEUEEMAILFLOW,
    #[serde(rename = "INQUEUESHORTMESSAGEFLOW")]
    INQUEUESHORTMESSAGEFLOW,
    #[serde(rename = "IVRCONFIGURATION")]
    IVRCONFIGURATION,
    #[serde(rename = "KNOWLEDGEBASE")]
    KNOWLEDGEBASE,
    #[serde(rename = "KNOWLEDGEBASEDOCUMENT")]
    KNOWLEDGEBASEDOCUMENT,
    #[serde(rename = "LANGUAGE")]
    LANGUAGE,
    #[serde(rename = "LEXBOT")]
    LEXBOT,
    #[serde(rename = "LEXBOTALIAS")]
    LEXBOTALIAS,
    #[serde(rename = "LEXV2BOT")]
    LEXV2BOT,
    #[serde(rename = "LEXV2BOTALIAS")]
    LEXV2BOTALIAS,
    #[serde(rename = "NLUDOMAIN")]
    NLUDOMAIN,
    #[serde(rename = "NUANCEMIXBOT")]
    NUANCEMIXBOT,
    #[serde(rename = "NUANCEMIXINTEGRATION")]
    NUANCEMIXINTEGRATION,
    #[serde(rename = "OAUTHCLIENT")]
    OAUTHCLIENT,
    #[serde(rename = "OUTBOUNDCALLFLOW")]
    OUTBOUNDCALLFLOW,
    #[serde(rename = "QUEUE")]
    QUEUE,
    #[serde(rename = "RECORDINGPOLICY")]
    RECORDINGPOLICY,
    #[serde(rename = "RESPONSE")]
    RESPONSE,
    #[serde(rename = "SCHEDULE")]
    SCHEDULE,
    #[serde(rename = "SCHEDULEGROUP")]
    SCHEDULEGROUP,
    #[serde(rename = "SECUREACTION")]
    SECUREACTION,
    #[serde(rename = "SECURECALLFLOW")]
    SECURECALLFLOW,
    #[serde(rename = "SURVEYINVITEFLOW")]
    SURVEYINVITEFLOW,
    #[serde(rename = "SYSTEMPROMPT")]
    SYSTEMPROMPT,
    #[serde(rename = "TTSENGINE")]
    TTSENGINE,
    #[serde(rename = "TTSVOICE")]
    TTSVOICE,
    #[serde(rename = "USER")]
    USER,
    #[serde(rename = "USERPROMPT")]
    USERPROMPT,
    #[serde(rename = "VOICEMAILFLOW")]
    VOICEMAILFLOW,
    #[serde(rename = "WIDGET")]
    WIDGET,
    #[serde(rename = "WORKFLOW")]
    WORKFLOW,
    #[serde(rename = "WORKITEMFLOW")]
    WORKITEMFLOW,
}

impl Default for Type {
    fn default() -> Type {
        Self::ACDLANGUAGE
    }
}

