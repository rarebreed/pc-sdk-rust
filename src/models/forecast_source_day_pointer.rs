/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */

/// ForecastSourceDayPointer : Pointer to look up source data for a short term forecast



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ForecastSourceDayPointer {
    /// The forecast day of week for this source data
    #[serde(rename = "dayOfWeek", skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,
    /// The relative weight to apply to this source data item for weighted averages
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// The date this source data represents, in yyyy-MM-dd format
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The name of the source file this data came from if it originated from a data import
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The key to look up the forecast source data for this source day
    #[serde(rename = "dataKey", skip_serializing_if = "Option::is_none")]
    pub data_key: Option<String>,
}

impl ForecastSourceDayPointer {
    /// Pointer to look up source data for a short term forecast
    pub fn new() -> ForecastSourceDayPointer {
        ForecastSourceDayPointer {
            day_of_week: None,
            weight: None,
            date: None,
            file_name: None,
            data_key: None,
        }
    }
}

/// The forecast day of week for this source data
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DayOfWeek {
    #[serde(rename = "Sunday")]
    Sunday,
    #[serde(rename = "Monday")]
    Monday,
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[serde(rename = "Wednesday")]
    Wednesday,
    #[serde(rename = "Thursday")]
    Thursday,
    #[serde(rename = "Friday")]
    Friday,
    #[serde(rename = "Saturday")]
    Saturday,
    #[serde(rename = "EighthDay")]
    EighthDay,
}

impl Default for DayOfWeek {
    fn default() -> DayOfWeek {
        Self::Sunday
    }
}
