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
pub struct ContactList {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// Required for updates, must match the version number of the most recent update
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::DomainEntityRef>>,
    /// The names of the contact data columns.
    #[serde(rename = "columnNames")]
    pub column_names: Vec<String>,
    /// Indicates which columns are phone numbers.
    #[serde(rename = "phoneColumns", skip_serializing_if = "Option::is_none")]
    pub phone_columns: Option<Vec<crate::models::ContactPhoneNumberColumn>>,
    #[serde(rename = "importStatus", skip_serializing_if = "Option::is_none")]
    pub import_status: Option<Box<crate::models::ImportStatus>>,
    /// A column to check if a contact should always be dialed in preview mode.
    #[serde(rename = "previewModeColumnName", skip_serializing_if = "Option::is_none")]
    pub preview_mode_column_name: Option<String>,
    /// The values in the previewModeColumnName column that indicate a contact should always be dialed in preview mode.
    #[serde(rename = "previewModeAcceptedValues", skip_serializing_if = "Option::is_none")]
    pub preview_mode_accepted_values: Option<Vec<String>>,
    /// The number of contacts in the ContactList.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "attemptLimits", skip_serializing_if = "Option::is_none")]
    pub attempt_limits: Option<Box<crate::models::DomainEntityRef>>,
    /// Indicates if automatic time zone mapping is to be used for this ContactList.
    #[serde(rename = "automaticTimeZoneMapping", skip_serializing_if = "Option::is_none")]
    pub automatic_time_zone_mapping: Option<bool>,
    /// The name of contact list column containing the zip code for use with automatic time zone mapping. Only allowed if 'automaticTimeZoneMapping' is set to true.
    #[serde(rename = "zipCodeColumnName", skip_serializing_if = "Option::is_none")]
    pub zip_code_column_name: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl ContactList {
    pub fn new(column_names: Vec<String>) -> ContactList {
        ContactList {
            id: None,
            name: None,
            date_created: None,
            date_modified: None,
            version: None,
            division: None,
            column_names,
            phone_columns: None,
            import_status: None,
            preview_mode_column_name: None,
            preview_mode_accepted_values: None,
            size: None,
            attempt_limits: None,
            automatic_time_zone_mapping: None,
            zip_code_column_name: None,
            self_uri: None,
        }
    }
}


