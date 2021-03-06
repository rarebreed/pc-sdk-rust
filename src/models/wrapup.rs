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
pub struct Wrapup {
    /// The user configured wrap up code id.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The user configured wrap up code name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Text entered by the agent to describe the call or disposition.
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// List of tags selected by the agent to describe the call or disposition.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The length of time in seconds that the agent spent doing after call work.
    #[serde(rename = "durationSeconds", skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    /// The timestamp when the wrapup was finished. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Indicates if this is a pending save and should not require a code to be specified.  This allows someone to save some temporary wrapup that will be used later.
    #[serde(rename = "provisional", skip_serializing_if = "Option::is_none")]
    pub provisional: Option<bool>,
}

impl Wrapup {
    pub fn new() -> Wrapup {
        Wrapup {
            code: None,
            name: None,
            notes: None,
            tags: None,
            duration_seconds: None,
            end_time: None,
            provisional: None,
        }
    }
}


