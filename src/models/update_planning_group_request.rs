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
pub struct UpdatePlanningGroupRequest {
    /// The name of the planning group
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routePaths", skip_serializing_if = "Option::is_none")]
    pub route_paths: Option<Box<crate::models::SetWrapperRoutePathRequest>>,
    /// The ID of the service goal template to associate with this planning group
    #[serde(rename = "serviceGoalTemplateId", skip_serializing_if = "Option::is_none")]
    pub service_goal_template_id: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::WfmVersionedEntityMetadata>,
}

impl UpdatePlanningGroupRequest {
    pub fn new(metadata: crate::models::WfmVersionedEntityMetadata) -> UpdatePlanningGroupRequest {
        UpdatePlanningGroupRequest {
            name: None,
            route_paths: None,
            service_goal_template_id: None,
            metadata: Box::new(metadata),
        }
    }
}


