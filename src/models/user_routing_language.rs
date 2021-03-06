/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// UserRoutingLanguage : Represents an organization language assigned to a user. When assigning to a user specify the organization language id as the id.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserRoutingLanguage {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A rating from 0.0 to 5.0 that indicates how fluent an agent is in a particular language. ACD interactions are routed to agents with higher proficiency ratings.
    #[serde(rename = "proficiency", skip_serializing_if = "Option::is_none")]
    pub proficiency: Option<f64>,
    /// Activate or deactivate this routing language.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// URI to the organization language used by this user language.
    #[serde(rename = "languageUri", skip_serializing_if = "Option::is_none")]
    pub language_uri: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl UserRoutingLanguage {
    /// Represents an organization language assigned to a user. When assigning to a user specify the organization language id as the id.
    pub fn new() -> UserRoutingLanguage {
        UserRoutingLanguage {
            id: None,
            name: None,
            proficiency: None,
            state: None,
            language_uri: None,
            self_uri: None,
        }
    }
}

/// Activate or deactivate this routing language.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}

