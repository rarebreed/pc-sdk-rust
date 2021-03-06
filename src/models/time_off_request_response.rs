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
pub struct TimeOffRequestResponse {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::UserReference>>,
    /// Whether this is a full day request (false means partial day)
    #[serde(rename = "isFullDayRequest", skip_serializing_if = "Option::is_none")]
    pub is_full_day_request: Option<bool>,
    /// Whether this request has been marked as read by the agent
    #[serde(rename = "markedAsRead", skip_serializing_if = "Option::is_none")]
    pub marked_as_read: Option<bool>,
    /// The ID of the activity code associated with this time off request. Activity code must be of the TimeOff category
    #[serde(rename = "activityCodeId", skip_serializing_if = "Option::is_none")]
    pub activity_code_id: Option<String>,
    /// The status of this time off request
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// A set of start date-times in ISO-8601 format for partial day requests.  Will be not empty if isFullDayRequest == false
    #[serde(rename = "partialDayStartDateTimes", skip_serializing_if = "Option::is_none")]
    pub partial_day_start_date_times: Option<Vec<String>>,
    /// A set of dates in yyyy-MM-dd format.  Should be interpreted in the management unit's configured time zone.  Will be not empty if isFullDayRequest == true
    #[serde(rename = "fullDayManagementUnitDates", skip_serializing_if = "Option::is_none")]
    pub full_day_management_unit_dates: Option<Vec<String>>,
    /// The daily duration of this time off request in minutes
    #[serde(rename = "dailyDurationMinutes", skip_serializing_if = "Option::is_none")]
    pub daily_duration_minutes: Option<i32>,
    /// Notes about the time off request
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "submittedBy", skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<Box<crate::models::UserReference>>,
    /// The timestamp when this request was submitted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "submittedDate", skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<String>,
    #[serde(rename = "reviewedBy", skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<Box<crate::models::UserReference>>,
    /// The timestamp when this request was reviewed. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "reviewedDate", skip_serializing_if = "Option::is_none")]
    pub reviewed_date: Option<String>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::UserReference>>,
    /// The timestamp when this request was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "modifiedDate", skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::WfmVersionedEntityMetadata>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl TimeOffRequestResponse {
    pub fn new() -> TimeOffRequestResponse {
        TimeOffRequestResponse {
            id: None,
            user: None,
            is_full_day_request: None,
            marked_as_read: None,
            activity_code_id: None,
            status: None,
            partial_day_start_date_times: None,
            full_day_management_unit_dates: None,
            daily_duration_minutes: None,
            notes: None,
            submitted_by: None,
            submitted_date: None,
            reviewed_by: None,
            reviewed_date: None,
            modified_by: None,
            modified_date: None,
            metadata: None,
            self_uri: None,
        }
    }
}

/// The status of this time off request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "APPROVED")]
    APPROVED,
    #[serde(rename = "DENIED")]
    DENIED,
    #[serde(rename = "CANCELED")]
    CANCELED,
}

impl Default for Status {
    fn default() -> Status {
        Self::PENDING
    }
}

