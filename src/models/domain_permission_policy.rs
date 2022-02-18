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
pub struct DomainPermissionPolicy {
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "entityName", skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(rename = "policyName", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "policyDescription", skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    #[serde(rename = "actionSet", skip_serializing_if = "Option::is_none")]
    pub action_set: Option<Vec<String>>,
    #[serde(rename = "namedResources", skip_serializing_if = "Option::is_none")]
    pub named_resources: Option<Vec<String>>,
    #[serde(rename = "allowConditions", skip_serializing_if = "Option::is_none")]
    pub allow_conditions: Option<bool>,
    #[serde(rename = "resourceConditionNode", skip_serializing_if = "Option::is_none")]
    pub resource_condition_node: Option<Box<crate::models::DomainResourceConditionNode>>,
}

impl DomainPermissionPolicy {
    pub fn new() -> DomainPermissionPolicy {
        DomainPermissionPolicy {
            domain: None,
            entity_name: None,
            policy_name: None,
            policy_description: None,
            action_set: None,
            named_resources: None,
            allow_conditions: None,
            resource_condition_node: None,
        }
    }
}


