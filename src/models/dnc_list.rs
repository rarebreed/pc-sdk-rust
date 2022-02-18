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
pub struct DncList {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the DncList.
    #[serde(rename = "name")]
    pub name: String,
    /// Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// Required for updates, must match the version number of the most recent update
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "importStatus", skip_serializing_if = "Option::is_none")]
    pub import_status: Option<Box<crate::models::ImportStatus>>,
    /// The total number of phone numbers in the DncList.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The type of the DncList.
    #[serde(rename = "dncSourceType")]
    pub dnc_source_type: DncSourceType,
    /// A dnc.com loginId. Required if the dncSourceType is dnc.com.
    #[serde(rename = "loginId", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    /// The list of dnc.com codes to be treated as DNC. Required if the dncSourceType is dnc.com.
    #[serde(rename = "dncCodes", skip_serializing_if = "Option::is_none")]
    pub dnc_codes: Option<Vec<String>>,
    /// A gryphon license number. Required if the dncSourceType is gryphon.
    #[serde(rename = "licenseId", skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::DomainEntityRef>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl DncList {
    pub fn new(name: String, dnc_source_type: DncSourceType) -> DncList {
        DncList {
            id: None,
            name,
            date_created: None,
            date_modified: None,
            version: None,
            import_status: None,
            size: None,
            dnc_source_type,
            login_id: None,
            dnc_codes: None,
            license_id: None,
            division: None,
            self_uri: None,
        }
    }
}

/// The type of the DncList.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DncSourceType {
    #[serde(rename = "rds")]
    Rds,
    #[serde(rename = "dnc.com")]
    DncCom,
    #[serde(rename = "gryphon")]
    Gryphon,
}

impl Default for DncSourceType {
    fn default() -> DncSourceType {
        Self::Rds
    }
}

