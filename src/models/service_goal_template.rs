/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceGoalTemplate : Service Goal Template



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceGoalTemplate {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serviceLevel", skip_serializing_if = "Option::is_none")]
    pub service_level: Option<Box<crate::models::BuServiceLevel>>,
    #[serde(rename = "averageSpeedOfAnswer", skip_serializing_if = "Option::is_none")]
    pub average_speed_of_answer: Option<Box<crate::models::BuAverageSpeedOfAnswer>>,
    #[serde(rename = "abandonRate", skip_serializing_if = "Option::is_none")]
    pub abandon_rate: Option<Box<crate::models::BuAbandonRate>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::WfmVersionedEntityMetadata>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl ServiceGoalTemplate {
    /// Service Goal Template
    pub fn new() -> ServiceGoalTemplate {
        ServiceGoalTemplate {
            id: None,
            name: None,
            service_level: None,
            average_speed_of_answer: None,
            abandon_rate: None,
            metadata: None,
            self_uri: None,
        }
    }
}

