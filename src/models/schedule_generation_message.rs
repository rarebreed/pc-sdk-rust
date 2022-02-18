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
pub struct ScheduleGenerationMessage {
    /// The type of the message
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The arguments describing the message
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<crate::models::SchedulerMessageArgument>>,
}

impl ScheduleGenerationMessage {
    pub fn new() -> ScheduleGenerationMessage {
        ScheduleGenerationMessage {
            _type: None,
            arguments: None,
        }
    }
}

/// The type of the message
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "AgentNotFound")]
    AgentNotFound,
    #[serde(rename = "AgentNotInSelectedManagementUnit")]
    AgentNotInSelectedManagementUnit,
    #[serde(rename = "AgentNotLicensed")]
    AgentNotLicensed,
    #[serde(rename = "AgentWithoutWorkPlan")]
    AgentWithoutWorkPlan,
    #[serde(rename = "WorkPlanNotEnabled")]
    WorkPlanNotEnabled,
    #[serde(rename = "WorkPlanNotFound")]
    WorkPlanNotFound,
    #[serde(rename = "AgentWithoutCapability")]
    AgentWithoutCapability,
    #[serde(rename = "NoNeedDays")]
    NoNeedDays,
    #[serde(rename = "UnableToProduceAgentSchedule")]
    UnableToProduceAgentSchedule,
    #[serde(rename = "UnableToScheduleMaxConsecutiveWorkingDaysFromAgentHistory")]
    UnableToScheduleMaxConsecutiveWorkingDaysFromAgentHistory,
    #[serde(rename = "UnableToScheduleMaxConsecutiveWorkingWeekendsFromAgentHistory")]
    UnableToScheduleMaxConsecutiveWorkingWeekendsFromAgentHistory,
    #[serde(rename = "UnableToScheduleMaxWeeklyPaidTimeFromTimeOff")]
    UnableToScheduleMaxWeeklyPaidTimeFromTimeOff,
    #[serde(rename = "UnableToScheduleMaxWorkDayPaidTimeFromTimeOff")]
    UnableToScheduleMaxWorkDayPaidTimeFromTimeOff,
    #[serde(rename = "UnableToScheduleMinIntershiftTimeFromAgentHistory")]
    UnableToScheduleMinIntershiftTimeFromAgentHistory,
    #[serde(rename = "UnableToScheduleMinIntershiftTimeFromDst")]
    UnableToScheduleMinIntershiftTimeFromDst,
    #[serde(rename = "UnableToScheduleMinShiftStartDistanceFromAgentHistory")]
    UnableToScheduleMinShiftStartDistanceFromAgentHistory,
    #[serde(rename = "UnableToScheduleMinShiftStartDistanceFromDst")]
    UnableToScheduleMinShiftStartDistanceFromDst,
    #[serde(rename = "UnableToScheduleMinWeeklyPaidTimeFromTimeOff")]
    UnableToScheduleMinWeeklyPaidTimeFromTimeOff,
    #[serde(rename = "UnableToScheduleMinWeeklyWorkDaysFromTimeOff")]
    UnableToScheduleMinWeeklyWorkDaysFromTimeOff,
    #[serde(rename = "UnableToScheduleMinWorkDayPaidTimeFromTimeOff")]
    UnableToScheduleMinWorkDayPaidTimeFromTimeOff,
    #[serde(rename = "UnableToSchedulePlanningPeriodMaxDaysOffFromAgentHistory")]
    UnableToSchedulePlanningPeriodMaxDaysOffFromAgentHistory,
    #[serde(rename = "UnableToSchedulePlanningPeriodMaxDaysOffFromTimeOff")]
    UnableToSchedulePlanningPeriodMaxDaysOffFromTimeOff,
    #[serde(rename = "UnableToSchedulePlanningPeriodMaxPaidTimeFromAgentHistory")]
    UnableToSchedulePlanningPeriodMaxPaidTimeFromAgentHistory,
    #[serde(rename = "UnableToSchedulePlanningPeriodMaxPaidTimeFromTimeOff")]
    UnableToSchedulePlanningPeriodMaxPaidTimeFromTimeOff,
    #[serde(rename = "UnableToSchedulePlanningPeriodMinDaysOffFromAgentHistory")]
    UnableToSchedulePlanningPeriodMinDaysOffFromAgentHistory,
    #[serde(rename = "UnableToSchedulePlanningPeriodMinPaidTimeFromAgentHistory")]
    UnableToSchedulePlanningPeriodMinPaidTimeFromAgentHistory,
    #[serde(rename = "UnableToSchedulePlanningPeriodMinPaidTimeFromTimeOff")]
    UnableToSchedulePlanningPeriodMinPaidTimeFromTimeOff,
    #[serde(rename = "UnableToScheduleWorkDayFromTimeOff")]
    UnableToScheduleWorkDayFromTimeOff,
    #[serde(rename = "UnableToScheduleMaxConsecutiveWorkingDays")]
    UnableToScheduleMaxConsecutiveWorkingDays,
    #[serde(rename = "UnableToScheduleMaxConsecutiveWorkingWeekends")]
    UnableToScheduleMaxConsecutiveWorkingWeekends,
    #[serde(rename = "UnableToScheduleMaxWeeklyPaidTime")]
    UnableToScheduleMaxWeeklyPaidTime,
    #[serde(rename = "UnableToScheduleMaxWeeklyWorkDays")]
    UnableToScheduleMaxWeeklyWorkDays,
    #[serde(rename = "UnableToScheduleMaxWorkDayPaidTime")]
    UnableToScheduleMaxWorkDayPaidTime,
    #[serde(rename = "UnableToScheduleMinConsecutiveNonWorkingTimePerWeek")]
    UnableToScheduleMinConsecutiveNonWorkingTimePerWeek,
    #[serde(rename = "UnableToScheduleMinIntershiftTime")]
    UnableToScheduleMinIntershiftTime,
    #[serde(rename = "UnableToScheduleMinShiftStartDistance")]
    UnableToScheduleMinShiftStartDistance,
    #[serde(rename = "UnableToScheduleMinWeeklyPaidTime")]
    UnableToScheduleMinWeeklyPaidTime,
    #[serde(rename = "UnableToScheduleMinWeeklyWorkDays")]
    UnableToScheduleMinWeeklyWorkDays,
    #[serde(rename = "UnableToScheduleMinWorkDayPaidTime")]
    UnableToScheduleMinWorkDayPaidTime,
    #[serde(rename = "UnableToSchedulePlanningPeriodMaxDaysOff")]
    UnableToSchedulePlanningPeriodMaxDaysOff,
    #[serde(rename = "UnableToSchedulePlanningPeriodMaxPaidTime")]
    UnableToSchedulePlanningPeriodMaxPaidTime,
    #[serde(rename = "UnableToSchedulePlanningPeriodMinDaysOff")]
    UnableToSchedulePlanningPeriodMinDaysOff,
    #[serde(rename = "UnableToSchedulePlanningPeriodMinPaidTime")]
    UnableToSchedulePlanningPeriodMinPaidTime,
    #[serde(rename = "UnableToScheduleShiftVariance")]
    UnableToScheduleShiftVariance,
    #[serde(rename = "UnableToScheduleWorkDay")]
    UnableToScheduleWorkDay,
}

impl Default for Type {
    fn default() -> Type {
        Self::AgentNotFound
    }
}

