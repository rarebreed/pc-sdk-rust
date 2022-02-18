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
pub struct MinLength {
    /// A non-negative integer for a text-based schema field denoting the minimum smallest length a string field can contain for a schema instance.
    #[serde(rename = "min")]
    pub min: i64,
    /// A non-negative integer for a text-based schema field denoting the maximum smallest length string the field can contain for a schema instance.
    #[serde(rename = "max")]
    pub max: i64,
}

impl MinLength {
    pub fn new(min: i64, max: i64) -> MinLength {
        MinLength {
            min,
            max,
        }
    }
}


