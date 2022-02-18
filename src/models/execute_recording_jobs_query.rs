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
pub struct ExecuteRecordingJobsQuery {
    /// The desired state for the job to be set to.
    #[serde(rename = "state")]
    pub state: State,
}

impl ExecuteRecordingJobsQuery {
    pub fn new(state: State) -> ExecuteRecordingJobsQuery {
        ExecuteRecordingJobsQuery {
            state,
        }
    }
}

/// The desired state for the job to be set to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "PROCESSING")]
    PROCESSING,
}

impl Default for State {
    fn default() -> State {
        Self::PROCESSING
    }
}
