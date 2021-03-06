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
pub struct HomerRecord {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// metadata associated to the SIP calls. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "milliTs", skip_serializing_if = "Option::is_none")]
    pub milli_ts: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "microTs", skip_serializing_if = "Option::is_none")]
    pub micro_ts: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "replyReason", skip_serializing_if = "Option::is_none")]
    pub reply_reason: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "ruri", skip_serializing_if = "Option::is_none")]
    pub ruri: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "ruriUser", skip_serializing_if = "Option::is_none")]
    pub ruri_user: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "ruriDomain", skip_serializing_if = "Option::is_none")]
    pub ruri_domain: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "fromUser", skip_serializing_if = "Option::is_none")]
    pub from_user: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "fromDomain", skip_serializing_if = "Option::is_none")]
    pub from_domain: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "fromTag", skip_serializing_if = "Option::is_none")]
    pub from_tag: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "toUser", skip_serializing_if = "Option::is_none")]
    pub to_user: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "toDomain", skip_serializing_if = "Option::is_none")]
    pub to_domain: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "toTag", skip_serializing_if = "Option::is_none")]
    pub to_tag: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "pidUser", skip_serializing_if = "Option::is_none")]
    pub pid_user: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "contactUser", skip_serializing_if = "Option::is_none")]
    pub contact_user: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "authUser", skip_serializing_if = "Option::is_none")]
    pub auth_user: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "callid", skip_serializing_if = "Option::is_none")]
    pub callid: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "callidAleg", skip_serializing_if = "Option::is_none")]
    pub callid_aleg: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "via1", skip_serializing_if = "Option::is_none")]
    pub via1: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "via1Branch", skip_serializing_if = "Option::is_none")]
    pub via1_branch: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "cseq", skip_serializing_if = "Option::is_none")]
    pub cseq: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "diversion", skip_serializing_if = "Option::is_none")]
    pub diversion: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "sourceIp", skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "sourcePort", skip_serializing_if = "Option::is_none")]
    pub source_port: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "destinationIp", skip_serializing_if = "Option::is_none")]
    pub destination_ip: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "destinationPort", skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "contactIp", skip_serializing_if = "Option::is_none")]
    pub contact_ip: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "contactPort", skip_serializing_if = "Option::is_none")]
    pub contact_port: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "originatorIp", skip_serializing_if = "Option::is_none")]
    pub originator_ip: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "originatorPort", skip_serializing_if = "Option::is_none")]
    pub originator_port: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "proto", skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "rtpStat", skip_serializing_if = "Option::is_none")]
    pub rtp_stat: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "node", skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "trans", skip_serializing_if = "Option::is_none")]
    pub trans: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "dbnode", skip_serializing_if = "Option::is_none")]
    pub dbnode: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "sourceAlias", skip_serializing_if = "Option::is_none")]
    pub source_alias: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "destinationAlias", skip_serializing_if = "Option::is_none")]
    pub destination_alias: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "conversationId", skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// metadata associated to the SIP calls
    #[serde(rename = "participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl HomerRecord {
    pub fn new() -> HomerRecord {
        HomerRecord {
            id: None,
            name: None,
            date: None,
            milli_ts: None,
            micro_ts: None,
            method: None,
            reply_reason: None,
            ruri: None,
            ruri_user: None,
            ruri_domain: None,
            from_user: None,
            from_domain: None,
            from_tag: None,
            to_user: None,
            to_domain: None,
            to_tag: None,
            pid_user: None,
            contact_user: None,
            auth_user: None,
            callid: None,
            callid_aleg: None,
            via1: None,
            via1_branch: None,
            cseq: None,
            diversion: None,
            reason: None,
            content_type: None,
            auth: None,
            user_agent: None,
            source_ip: None,
            source_port: None,
            destination_ip: None,
            destination_port: None,
            contact_ip: None,
            contact_port: None,
            originator_ip: None,
            originator_port: None,
            correlation_id: None,
            proto: None,
            family: None,
            rtp_stat: None,
            _type: None,
            node: None,
            trans: None,
            dbnode: None,
            msg: None,
            source_alias: None,
            destination_alias: None,
            conversation_id: None,
            participant_id: None,
            self_uri: None,
        }
    }
}


