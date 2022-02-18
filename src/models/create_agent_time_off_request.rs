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
pub struct CreateAgentTimeOffRequest {
    /// The ID of the activity code associated with this time off request. Activity code must be of the TimeOff category
    #[serde(rename = "activityCodeId")]
    pub activity_code_id: String,
    /// Notes about the time off request
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// A set of dates in yyyy-MM-dd format.  Should be interpreted in the management unit's configured time zone.
    #[serde(rename = "fullDayManagementUnitDates", skip_serializing_if = "Option::is_none")]
    pub full_day_management_unit_dates: Option<Vec<String>>,
    /// A set of start date-times in ISO-8601 format for partial day requests.
    #[serde(rename = "partialDayStartDateTimes", skip_serializing_if = "Option::is_none")]
    pub partial_day_start_date_times: Option<Vec<String>>,
    /// The daily duration of this time off request in minutes
    #[serde(rename = "dailyDurationMinutes")]
    pub daily_duration_minutes: i32,
}

impl CreateAgentTimeOffRequest {
    pub fn new(activity_code_id: String, daily_duration_minutes: i32) -> CreateAgentTimeOffRequest {
        CreateAgentTimeOffRequest {
            activity_code_id,
            notes: None,
            full_day_management_unit_dates: None,
            partial_day_start_date_times: None,
            daily_duration_minutes,
        }
    }
}

