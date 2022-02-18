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
pub struct SchedulingNoForecastOptionsRequest {
    /// The shift length option to apply if no forecast is supplied
    #[serde(rename = "shiftLength", skip_serializing_if = "Option::is_none")]
    pub shift_length: Option<ShiftLength>,
    /// The shift start option to apply if no forecast is supplied
    #[serde(rename = "shiftStart", skip_serializing_if = "Option::is_none")]
    pub shift_start: Option<ShiftStart>,
}

impl SchedulingNoForecastOptionsRequest {
    pub fn new() -> SchedulingNoForecastOptionsRequest {
        SchedulingNoForecastOptionsRequest {
            shift_length: None,
            shift_start: None,
        }
    }
}

/// The shift length option to apply if no forecast is supplied
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShiftLength {
    #[serde(rename = "Shortest")]
    Shortest,
    #[serde(rename = "Median")]
    Median,
    #[serde(rename = "Longest")]
    Longest,
    #[serde(rename = "Random")]
    Random,
}

impl Default for ShiftLength {
    fn default() -> ShiftLength {
        Self::Shortest
    }
}
/// The shift start option to apply if no forecast is supplied
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShiftStart {
    #[serde(rename = "Earliest")]
    Earliest,
    #[serde(rename = "Median")]
    Median,
    #[serde(rename = "Latest")]
    Latest,
    #[serde(rename = "Random")]
    Random,
}

impl Default for ShiftStart {
    fn default() -> ShiftStart {
        Self::Earliest
    }
}
