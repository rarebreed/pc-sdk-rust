/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// UserSchedule : A schedule for a single user over a given time range



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserSchedule {
    /// The shifts that belong to this schedule
    #[serde(rename = "shifts", skip_serializing_if = "Option::is_none")]
    pub shifts: Option<Vec<crate::models::UserScheduleShift>>,
    /// Markers to indicate a full day time off request, relative to the management unit time zone
    #[serde(rename = "fullDayTimeOffMarkers", skip_serializing_if = "Option::is_none")]
    pub full_day_time_off_markers: Option<Vec<crate::models::UserScheduleFullDayTimeOffMarker>>,
    /// If marked true for updating an existing user schedule, it will be deleted
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::WfmVersionedEntityMetadata>,
    /// ID of the work plan associated with the user during schedule creation
    #[serde(rename = "workPlanId", skip_serializing_if = "Option::is_none")]
    pub work_plan_id: Option<String>,
}

impl UserSchedule {
    /// A schedule for a single user over a given time range
    pub fn new(metadata: crate::models::WfmVersionedEntityMetadata) -> UserSchedule {
        UserSchedule {
            shifts: None,
            full_day_time_off_markers: None,
            delete: None,
            metadata: Box::new(metadata),
            work_plan_id: None,
        }
    }
}


