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
pub struct AnalyticsParticipantWithoutAttributes {
    /// External contact identifier
    #[serde(rename = "externalContactId", skip_serializing_if = "Option::is_none")]
    pub external_contact_id: Option<String>,
    /// External organization identifier
    #[serde(rename = "externalOrganizationId", skip_serializing_if = "Option::is_none")]
    pub external_organization_id: Option<String>,
    /// Reason for which participant flagged conversation
    #[serde(rename = "flaggedReason", skip_serializing_if = "Option::is_none")]
    pub flagged_reason: Option<FlaggedReason>,
    /// Unique identifier for the participant
    #[serde(rename = "participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    /// A human readable name identifying the participant
    #[serde(rename = "participantName", skip_serializing_if = "Option::is_none")]
    pub participant_name: Option<String>,
    /// The participant's purpose
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Purpose>,
    /// The team ID the user is a member of
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Unique identifier for the user
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// List of sessions associated to this participant
    #[serde(rename = "sessions", skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<crate::models::AnalyticsSession>>,
}

impl AnalyticsParticipantWithoutAttributes {
    pub fn new() -> AnalyticsParticipantWithoutAttributes {
        AnalyticsParticipantWithoutAttributes {
            external_contact_id: None,
            external_organization_id: None,
            flagged_reason: None,
            participant_id: None,
            participant_name: None,
            purpose: None,
            team_id: None,
            user_id: None,
            sessions: None,
        }
    }
}

/// Reason for which participant flagged conversation
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlaggedReason {
    #[serde(rename = "general")]
    General,
}

impl Default for FlaggedReason {
    fn default() -> FlaggedReason {
        Self::General
    }
}
/// The participant's purpose
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Purpose {
    #[serde(rename = "acd")]
    Acd,
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "botflow")]
    Botflow,
    #[serde(rename = "campaign")]
    Campaign,
    #[serde(rename = "customer")]
    Customer,
    #[serde(rename = "dialer")]
    Dialer,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "fax")]
    Fax,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "ivr")]
    Ivr,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "voicemail")]
    Voicemail,
    #[serde(rename = "workflow")]
    Workflow,
}

impl Default for Purpose {
    fn default() -> Purpose {
        Self::Acd
    }
}

