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
pub struct ContentFilterItem {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl ContentFilterItem {
    pub fn new() -> ContentFilterItem {
        ContentFilterItem {
            name: None,
            _type: None,
            operator: None,
            values: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "NUMBER")]
    NUMBER,
    #[serde(rename = "STRING")]
    STRING,
    #[serde(rename = "DATE")]
    DATE,
    #[serde(rename = "BOOLEAN")]
    BOOLEAN,
    #[serde(rename = "LIST")]
    LIST,
}

impl Default for Type {
    fn default() -> Type {
        Self::NUMBER
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "IN")]
    _IN,
    #[serde(rename = "RANGE")]
    RANGE,
    #[serde(rename = "EQUALS")]
    EQUALS,
    #[serde(rename = "NOTEQUALS")]
    NOTEQUALS,
    #[serde(rename = "LESSTHAN")]
    LESSTHAN,
    #[serde(rename = "LESSTHANEQUALS")]
    LESSTHANEQUALS,
    #[serde(rename = "GREATERTHAN")]
    GREATERTHAN,
    #[serde(rename = "GREATERTHANEQUALS")]
    GREATERTHANEQUALS,
    #[serde(rename = "CONTAINS")]
    CONTAINS,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::_IN
    }
}

