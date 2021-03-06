/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceGoalTemplateList : List of service goal templates



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceGoalTemplateList {
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::ServiceGoalTemplate>>,
}

impl ServiceGoalTemplateList {
    /// List of service goal templates
    pub fn new() -> ServiceGoalTemplateList {
        ServiceGoalTemplateList {
            entities: None,
        }
    }
}


