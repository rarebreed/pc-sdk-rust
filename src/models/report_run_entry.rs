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
pub struct ReportRunEntry {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "reportId", skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "runTime", skip_serializing_if = "Option::is_none")]
    pub run_time: Option<String>,
    #[serde(rename = "runStatus", skip_serializing_if = "Option::is_none")]
    pub run_status: Option<RunStatus>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "runDurationMsec", skip_serializing_if = "Option::is_none")]
    pub run_duration_msec: Option<i64>,
    #[serde(rename = "reportUrl", skip_serializing_if = "Option::is_none")]
    pub report_url: Option<String>,
    #[serde(rename = "reportFormat", skip_serializing_if = "Option::is_none")]
    pub report_format: Option<String>,
    #[serde(rename = "scheduleUri", skip_serializing_if = "Option::is_none")]
    pub schedule_uri: Option<String>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl ReportRunEntry {
    pub fn new() -> ReportRunEntry {
        ReportRunEntry {
            id: None,
            name: None,
            report_id: None,
            run_time: None,
            run_status: None,
            error_message: None,
            run_duration_msec: None,
            report_url: None,
            report_format: None,
            schedule_uri: None,
            self_uri: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RunStatus {
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "COMPLETED")]
    COMPLETED,
    #[serde(rename = "COMPLETED_WITH_ERRORS")]
    COMPLETEDWITHERRORS,
    #[serde(rename = "FAILED")]
    FAILED,
    #[serde(rename = "FAILED_TIMEOUT")]
    FAILEDTIMEOUT,
    #[serde(rename = "FAILED_DATALIMIT")]
    FAILEDDATALIMIT,
    #[serde(rename = "UNABLE_TO_COMPLETE")]
    UNABLETOCOMPLETE,
}

impl Default for RunStatus {
    fn default() -> RunStatus {
        Self::RUNNING
    }
}
