/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// HeadcountInterval : Headcount interval information for schedule



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HeadcountInterval {
    /// The start date-time for this headcount interval in ISO-8601 format, must be within the 8 day schedule
    #[serde(rename = "interval")]
    pub interval: String,
    /// Headcount value for this interval
    #[serde(rename = "value")]
    pub value: f64,
}

impl HeadcountInterval {
    /// Headcount interval information for schedule
    pub fn new(interval: String, value: f64) -> HeadcountInterval {
        HeadcountInterval {
            interval,
            value,
        }
    }
}


