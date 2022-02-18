/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// SchedulingSettingsRequest : Scheduling Settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SchedulingSettingsRequest {
    /// Max occupancy percent for deferred work
    #[serde(rename = "maxOccupancyPercentForDeferredWork", skip_serializing_if = "Option::is_none")]
    pub max_occupancy_percent_for_deferred_work: Option<i32>,
    /// Default shrinkage percent for scheduling
    #[serde(rename = "defaultShrinkagePercent", skip_serializing_if = "Option::is_none")]
    pub default_shrinkage_percent: Option<f64>,
    #[serde(rename = "shrinkageOverrides", skip_serializing_if = "Option::is_none")]
    pub shrinkage_overrides: Option<Box<crate::models::ShrinkageOverrides>>,
    #[serde(rename = "planningPeriod", skip_serializing_if = "Option::is_none")]
    pub planning_period: Option<Box<crate::models::ValueWrapperPlanningPeriodSettings>>,
    /// Start day of weekend for scheduling
    #[serde(rename = "startDayOfWeekend", skip_serializing_if = "Option::is_none")]
    pub start_day_of_weekend: Option<StartDayOfWeekend>,
}

impl SchedulingSettingsRequest {
    /// Scheduling Settings
    pub fn new() -> SchedulingSettingsRequest {
        SchedulingSettingsRequest {
            max_occupancy_percent_for_deferred_work: None,
            default_shrinkage_percent: None,
            shrinkage_overrides: None,
            planning_period: None,
            start_day_of_weekend: None,
        }
    }
}

/// Start day of weekend for scheduling
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StartDayOfWeekend {
    #[serde(rename = "Sunday")]
    Sunday,
    #[serde(rename = "Monday")]
    Monday,
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[serde(rename = "Wednesday")]
    Wednesday,
    #[serde(rename = "Thursday")]
    Thursday,
    #[serde(rename = "Friday")]
    Friday,
    #[serde(rename = "Saturday")]
    Saturday,
}

impl Default for StartDayOfWeekend {
    fn default() -> StartDayOfWeekend {
        Self::Sunday
    }
}

