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
pub struct CoachingSlot {
    /// Start date and time of scheduled coaching appointment slot. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateStart", skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,
    /// Length of coaching appointment slot in minutes
    #[serde(rename = "lengthInMinutes", skip_serializing_if = "Option::is_none")]
    pub length_in_minutes: Option<i32>,
    /// Difference between scheduled and forecast headcount for this slot after scheduling the coaching appointment
    #[serde(rename = "staffingDifference", skip_serializing_if = "Option::is_none")]
    pub staffing_difference: Option<f64>,
    /// Rating based on the staffing difference for scheduled slot
    #[serde(rename = "differenceRating", skip_serializing_if = "Option::is_none")]
    pub difference_rating: Option<DifferenceRating>,
    #[serde(rename = "wfmSchedule", skip_serializing_if = "Option::is_none")]
    pub wfm_schedule: Option<Box<crate::models::WfmScheduleReference>>,
}

impl CoachingSlot {
    pub fn new() -> CoachingSlot {
        CoachingSlot {
            date_start: None,
            length_in_minutes: None,
            staffing_difference: None,
            difference_rating: None,
            wfm_schedule: None,
        }
    }
}

/// Rating based on the staffing difference for scheduled slot
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DifferenceRating {
    #[serde(rename = "Poor")]
    Poor,
    #[serde(rename = "Neutral")]
    Neutral,
    #[serde(rename = "Good")]
    Good,
}

impl Default for DifferenceRating {
    fn default() -> DifferenceRating {
        Self::Poor
    }
}

