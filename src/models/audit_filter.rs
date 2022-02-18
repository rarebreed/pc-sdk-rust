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
pub struct AuditFilter {
    /// The name of the field by which to filter.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the filter, DATE or STRING.
    #[serde(rename = "type")]
    pub _type: String,
    /// The operation that the filter performs.
    #[serde(rename = "operator")]
    pub operator: String,
    /// The values to make the filter comparison against.
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

impl AuditFilter {
    pub fn new(name: String, _type: String, operator: String, values: Vec<String>) -> AuditFilter {
        AuditFilter {
            name,
            _type,
            operator,
            values,
        }
    }
}

