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
pub struct ActionMapScheduleGroups {
    #[serde(rename = "actionMapScheduleGroup")]
    pub action_map_schedule_group: Box<crate::models::ActionMapScheduleGroup>,
    #[serde(rename = "emergencyActionMapScheduleGroup", skip_serializing_if = "Option::is_none")]
    pub emergency_action_map_schedule_group: Option<Box<crate::models::ActionMapScheduleGroup>>,
}

impl ActionMapScheduleGroups {
    pub fn new(action_map_schedule_group: crate::models::ActionMapScheduleGroup) -> ActionMapScheduleGroups {
        ActionMapScheduleGroups {
            action_map_schedule_group: Box::new(action_map_schedule_group),
            emergency_action_map_schedule_group: None,
        }
    }
}


