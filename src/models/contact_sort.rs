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
pub struct ContactSort {
    #[serde(rename = "fieldName", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// The direction in which to sort contacts.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// Whether or not the column contains numeric data.
    #[serde(rename = "numeric", skip_serializing_if = "Option::is_none")]
    pub numeric: Option<bool>,
}

impl ContactSort {
    pub fn new() -> ContactSort {
        ContactSort {
            field_name: None,
            direction: None,
            numeric: None,
        }
    }
}

/// The direction in which to sort contacts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "ASC")]
    ASC,
    #[serde(rename = "DESC")]
    DESC,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::ASC
    }
}

