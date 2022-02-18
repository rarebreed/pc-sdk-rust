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
pub struct CoachingAppointmentStatusRequest {
    /// The status of the coaching appointment
    #[serde(rename = "status")]
    pub status: Status,
}

impl CoachingAppointmentStatusRequest {
    pub fn new(status: Status) -> CoachingAppointmentStatusRequest {
        CoachingAppointmentStatusRequest {
            status,
        }
    }
}

/// The status of the coaching appointment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Scheduled")]
    Scheduled,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Scheduled
    }
}

