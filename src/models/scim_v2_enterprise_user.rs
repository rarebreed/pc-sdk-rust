/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ScimV2EnterpriseUser : Defines a SCIM enterprise user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimV2EnterpriseUser {
    /// The division that the user belongs to.
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<String>,
    /// The department that the user belongs to.
    #[serde(rename = "department", skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "manager", skip_serializing_if = "Option::is_none")]
    pub manager: Option<Box<crate::models::Manager>>,
    /// The user's employee number.
    #[serde(rename = "employeeNumber", skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
}

impl ScimV2EnterpriseUser {
    /// Defines a SCIM enterprise user.
    pub fn new() -> ScimV2EnterpriseUser {
        ScimV2EnterpriseUser {
            division: None,
            department: None,
            manager: None,
            employee_number: None,
        }
    }
}

