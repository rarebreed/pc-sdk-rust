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
pub struct EventMessage {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "messageWithParams", skip_serializing_if = "Option::is_none")]
    pub message_with_params: Option<String>,
    #[serde(rename = "messageParams", skip_serializing_if = "Option::is_none")]
    pub message_params: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "documentationUri", skip_serializing_if = "Option::is_none")]
    pub documentation_uri: Option<String>,
    #[serde(rename = "resourceURIs", skip_serializing_if = "Option::is_none")]
    pub resource_uris: Option<Vec<String>>,
}

impl EventMessage {
    pub fn new() -> EventMessage {
        EventMessage {
            code: None,
            message: None,
            message_with_params: None,
            message_params: None,
            documentation_uri: None,
            resource_uris: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "APPROACHING_CONTACT_LIMIT")]
    APPROACHINGCONTACTLIMIT,
    #[serde(rename = "APPROACHING_DNC_LIST_PHONE_NUMBER_LIMIT")]
    APPROACHINGDNCLISTPHONENUMBERLIMIT,
    #[serde(rename = "APPROACHING_DNC_ORGANIZATION_PHONE_NUMBER_LIMIT")]
    APPROACHINGDNCORGANIZATIONPHONENUMBERLIMIT,
    #[serde(rename = "APPROACHING_ENTITY_LIMIT")]
    APPROACHINGENTITYLIMIT,
    #[serde(rename = "AUTOMATIC_TIME_ZONE_ZIP_CODE_INVALID")]
    AUTOMATICTIMEZONEZIPCODEINVALID,
    #[serde(rename = "CAMPAIGN_CONTENT_TEMPLATE_SUBSTITUTION_MISMATCH")]
    CAMPAIGNCONTENTTEMPLATESUBSTITUTIONMISMATCH,
    #[serde(rename = "CAMPAIGN_EMAIL_BODY_CHARACTER_LIMIT_EXCEEDED")]
    CAMPAIGNEMAILBODYCHARACTERLIMITEXCEEDED,
    #[serde(rename = "CAMPAIGN_EMAIL_SUBJECT_CHARACTER_LIMIT_EXCEEDED")]
    CAMPAIGNEMAILSUBJECTCHARACTERLIMITEXCEEDED,
    #[serde(rename = "CAMPAIGN_MESSAGE_CHARACTER_LIMIT_EXCEEDED")]
    CAMPAIGNMESSAGECHARACTERLIMITEXCEEDED,
    #[serde(rename = "CAMPAIGN_START_ERROR")]
    CAMPAIGNSTARTERROR,
    #[serde(rename = "CAMPAIGN_RULE_START_ERROR")]
    CAMPAIGNRULESTARTERROR,
    #[serde(rename = "CAMPAIGN_SET_DIALING_MODE_ERROR")]
    CAMPAIGNSETDIALINGMODEERROR,
    #[serde(rename = "CAMPAIGN_STOPPED")]
    CAMPAIGNSTOPPED,
    #[serde(rename = "CAMPAIGN_THROTTLED")]
    CAMPAIGNTHROTTLED,
    #[serde(rename = "CAMPAIGN_QUEUE_MEMBERS_LIMIT_EXCEEDED")]
    CAMPAIGNQUEUEMEMBERSLIMITEXCEEDED,
    #[serde(rename = "INVALID_CALLABLE_TIME_ZONE")]
    INVALIDCALLABLETIMEZONE,
    #[serde(rename = "CALLBACK_CREATION_INVALID_NUMBER")]
    CALLBACKCREATIONINVALIDNUMBER,
    #[serde(rename = "CALL_RULE_INVALID_CONTACT_COLUMN")]
    CALLRULEINVALIDCONTACTCOLUMN,
    #[serde(rename = "CALL_RULE_MISSING_DATA_ACTION_INPUT")]
    CALLRULEMISSINGDATAACTIONINPUT,
    #[serde(rename = "CALL_RULE_MISMATCH_TYPE")]
    CALLRULEMISMATCHTYPE,
    #[serde(rename = "CALL_RULE_INVALID_OPERATOR")]
    CALLRULEINVALIDOPERATOR,
    #[serde(rename = "CALL_RULE_NO_DNC_LISTS_CONFIGURED")]
    CALLRULENODNCLISTSCONFIGURED,
    #[serde(rename = "CALL_RULE_UPDATED_PHONE_COLUMN")]
    CALLRULEUPDATEDPHONECOLUMN,
    #[serde(rename = "CONTACT_LIST_FILTER_EVALUATION_FAILED")]
    CONTACTLISTFILTEREVALUATIONFAILED,
    #[serde(rename = "CONTACT_LIST_FILTER_INTERNAL_ERROR")]
    CONTACTLISTFILTERINTERNALERROR,
    #[serde(rename = "CONTACT_COLUMNS_LIMIT_EXCEEDED")]
    CONTACTCOLUMNSLIMITEXCEEDED,
    #[serde(rename = "CONTACT_COLUMN_LENGTH_LIMIT_EXCEEDED")]
    CONTACTCOLUMNLENGTHLIMITEXCEEDED,
    #[serde(rename = "CONTACT_ID_LENGTH_LIMIT_EXCEEDED")]
    CONTACTIDLENGTHLIMITEXCEEDED,
    #[serde(rename = "CONTACT_DATUM_LENGTH_LIMIT_EXCEEDED")]
    CONTACTDATUMLENGTHLIMITEXCEEDED,
    #[serde(rename = "CONTACT_ZIP_CODE_COLUMN_VALUE_INVALID")]
    CONTACTZIPCODECOLUMNVALUEINVALID,
    #[serde(rename = "DATA_ACTION_EXECUTION_FAILED")]
    DATAACTIONEXECUTIONFAILED,
    #[serde(rename = "DATA_ACTION_AUTHENTICATION_FAILURE")]
    DATAACTIONAUTHENTICATIONFAILURE,
    #[serde(rename = "DATA_ACTION_NOT_FOUND")]
    DATAACTIONNOTFOUND,
    #[serde(rename = "DNC_AUTHENTICATION_FAILURE")]
    DNCAUTHENTICATIONFAILURE,
    #[serde(rename = "EXCEEDED_CONTACT_LIMIT")]
    EXCEEDEDCONTACTLIMIT,
    #[serde(rename = "EXCEEDED_DNC_RECORD_LIMIT")]
    EXCEEDEDDNCRECORDLIMIT,
    #[serde(rename = "EXCEEDED_DNC_PHONE_NUMBER_LENGTH")]
    EXCEEDEDDNCPHONENUMBERLENGTH,
    #[serde(rename = "INACTIVE_EDGES_FAILED_PLACE_CALLS")]
    INACTIVEEDGESFAILEDPLACECALLS,
    #[serde(rename = "INACTIVE_EDGES_TURNED_CAMPAIGN_OFF")]
    INACTIVEEDGESTURNEDCAMPAIGNOFF,
    #[serde(rename = "INVALID_AGENT")]
    INVALIDAGENT,
    #[serde(rename = "INVALID_EMAIL_ADDRESS")]
    INVALIDEMAILADDRESS,
    #[serde(rename = "INVALID_PHONE_NUMBER")]
    INVALIDPHONENUMBER,
    #[serde(rename = "IMPORT_FAILED_TO_READ_HEADERS")]
    IMPORTFAILEDTOREADHEADERS,
    #[serde(rename = "IMPORT_COULD_NOT_PARSE_AN_ENTRY")]
    IMPORTCOULDNOTPARSEANENTRY,
    #[serde(rename = "IMPORT_CONTACT_DOES_NOT_MATCH_LIST_FORMAT")]
    IMPORTCONTACTDOESNOTMATCHLISTFORMAT,
    #[serde(rename = "IMPORT_ENTRY_DOES_NOT_ALIGN_WITH_HEADERS")]
    IMPORTENTRYDOESNOTALIGNWITHHEADERS,
    #[serde(rename = "IMPORT_INVALID_CUSTOM_ID")]
    IMPORTINVALIDCUSTOMID,
    #[serde(rename = "IMPORT_INVALID_DATA")]
    IMPORTINVALIDDATA,
    #[serde(rename = "IMPORT_INVALID_EMAIL_ADDRESSES")]
    IMPORTINVALIDEMAILADDRESSES,
    #[serde(rename = "IMPORT_INVALID_PHONE_NUMBERS")]
    IMPORTINVALIDPHONENUMBERS,
    #[serde(rename = "IMPORT_INVALID_EXPIRATION_DATE")]
    IMPORTINVALIDEXPIRATIONDATE,
    #[serde(rename = "IMPORT_EXPIRATION_DATE_EXCEEDS_MAX_DAYS")]
    IMPORTEXPIRATIONDATEEXCEEDSMAXDAYS,
    #[serde(rename = "IMPORT_COLUMN_EXCEEDS_LENGTH_LIMIT")]
    IMPORTCOLUMNEXCEEDSLENGTHLIMIT,
    #[serde(rename = "IMPORT_DATUM_EXCEEDS_LENGTH_LIMIT")]
    IMPORTDATUMEXCEEDSLENGTHLIMIT,
    #[serde(rename = "IMPORT_MISSING_CUSTOM_ID")]
    IMPORTMISSINGCUSTOMID,
    #[serde(rename = "IMPORT_NO_COLUMNS_DEFINED")]
    IMPORTNOCOLUMNSDEFINED,
    #[serde(rename = "IMPORT_COLUMNS_DO_NOT_EXIST_ON_LIST")]
    IMPORTCOLUMNSDONOTEXISTONLIST,
    #[serde(rename = "IMPORT_LIST_NO_LONGER_EXISTS")]
    IMPORTLISTNOLONGEREXISTS,
    #[serde(rename = "IMPORT_FAILED_CONTACT_ZIP_CODE_COLUMN_VALUE_INVALID")]
    IMPORTFAILEDCONTACTZIPCODECOLUMNVALUEINVALID,
    #[serde(rename = "IMPORT_TOO_MANY_COLUMNS")]
    IMPORTTOOMANYCOLUMNS,
    #[serde(rename = "IMPORT_TOO_MANY_EXTRA_COLUMNS")]
    IMPORTTOOMANYEXTRACOLUMNS,
    #[serde(rename = "ORGANIZATION_HAS_NO_DOMAIN_SET")]
    ORGANIZATIONHASNODOMAINSET,
    #[serde(rename = "RECYCLE_CAMPAIGN")]
    RECYCLECAMPAIGN,
}

impl Default for Code {
    fn default() -> Code {
        Self::APPROACHINGCONTACTLIMIT
    }
}
