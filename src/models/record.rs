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
pub struct Record {
    /// The name of the record.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the record. (Example values:  MX, TXT, CNAME)
    #[serde(rename = "type")]
    pub _type: String,
    /// The value of the record.
    #[serde(rename = "value")]
    pub value: String,
}

impl Record {
    pub fn new(name: String, _type: String, value: String) -> Record {
        Record {
            name,
            _type,
            value,
        }
    }
}

