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
pub struct SessionSegmentAssignment {
    #[serde(rename = "segment", skip_serializing_if = "Option::is_none")]
    pub segment: Option<Box<crate::models::AssignedSegment>>,
    /// Timestamp indicating when the segment was assigned. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "assignedDate", skip_serializing_if = "Option::is_none")]
    pub assigned_date: Option<String>,
}

impl SessionSegmentAssignment {
    pub fn new() -> SessionSegmentAssignment {
        SessionSegmentAssignment {
            segment: None,
            assigned_date: None,
        }
    }
}

