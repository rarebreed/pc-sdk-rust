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
pub struct BillingUsageResource {
    /// Identifies the resource (e.g. license user, device).
    #[serde(rename = "name")]
    pub name: String,
    /// The date that the usage was first observed by the billing subsystem. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "date")]
    pub date: String,
}

impl BillingUsageResource {
    pub fn new(name: String, date: String) -> BillingUsageResource {
        BillingUsageResource {
            name,
            date,
        }
    }
}


