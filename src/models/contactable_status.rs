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
pub struct ContactableStatus {
    /// Indicates whether or not the entire contact is contactable for the associated media type.
    #[serde(rename = "contactable", skip_serializing_if = "Option::is_none")]
    pub contactable: Option<bool>,
    /// A map of individual contact method columns to whether the individual column is contactable for the associated media type.
    #[serde(rename = "columnStatus", skip_serializing_if = "Option::is_none")]
    pub column_status: Option<::std::collections::HashMap<String, crate::models::ColumnStatus>>,
}

impl ContactableStatus {
    pub fn new() -> ContactableStatus {
        ContactableStatus {
            contactable: None,
            column_status: None,
        }
    }
}


