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
pub struct EventLog {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "errorEntity", skip_serializing_if = "Option::is_none")]
    pub error_entity: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "relatedEntity", skip_serializing_if = "Option::is_none")]
    pub related_entity: Option<Box<crate::models::DomainEntityRef>>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(rename = "eventMessage", skip_serializing_if = "Option::is_none")]
    pub event_message: Option<Box<crate::models::EventMessage>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl EventLog {
    pub fn new() -> EventLog {
        EventLog {
            id: None,
            name: None,
            error_entity: None,
            related_entity: None,
            timestamp: None,
            level: None,
            category: None,
            correlation_id: None,
            event_message: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "INFO")]
    INFO,
    #[serde(rename = "WARNING")]
    WARNING,
    #[serde(rename = "ERROR")]
    ERROR,
}

impl Default for Level {
    fn default() -> Level {
        Self::INFO
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "CALLBACK")]
    CALLBACK,
    #[serde(rename = "CALL_RESTRICTION")]
    CALLRESTRICTION,
    #[serde(rename = "CALL_RULE")]
    CALLRULE,
    #[serde(rename = "CAMPAIGN")]
    CAMPAIGN,
    #[serde(rename = "CAMPAIGN_RULE")]
    CAMPAIGNRULE,
    #[serde(rename = "CONTACT")]
    CONTACT,
    #[serde(rename = "CONTACT_LIST_FILTER")]
    CONTACTLISTFILTER,
    #[serde(rename = "DNC_LIST")]
    DNCLIST,
    #[serde(rename = "ENTITY_LIMIT")]
    ENTITYLIMIT,
    #[serde(rename = "IMPORT_ERROR")]
    IMPORTERROR,
    #[serde(rename = "MESSAGE_RESTRICTION")]
    MESSAGERESTRICTION,
    #[serde(rename = "MESSAGING_CAMPAIGN")]
    MESSAGINGCAMPAIGN,
    #[serde(rename = "ORGANIZATION_CONFIGURATION")]
    ORGANIZATIONCONFIGURATION,
    #[serde(rename = "SCHEDULE")]
    SCHEDULE,
    #[serde(rename = "MESSAGING_CAMPAIGN_SCHEDULE")]
    MESSAGINGCAMPAIGNSCHEDULE,
    #[serde(rename = "EMAIL_CAMPAIGN_SCHEDULE")]
    EMAILCAMPAIGNSCHEDULE,
}

impl Default for Category {
    fn default() -> Category {
        Self::CALLBACK
    }
}

