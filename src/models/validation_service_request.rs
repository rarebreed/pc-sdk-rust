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
pub struct ValidationServiceRequest {
    /// The last day of the data you are importing. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateImportEnded")]
    pub date_import_ended: String,
    /// S3 key for the uploaded file
    #[serde(rename = "uploadKey")]
    pub upload_key: String,
}

impl ValidationServiceRequest {
    pub fn new(date_import_ended: String, upload_key: String) -> ValidationServiceRequest {
        ValidationServiceRequest {
            date_import_ended,
            upload_key,
        }
    }
}

