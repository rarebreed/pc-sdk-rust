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
pub struct ExternalOrganizationTrustorLink {
    /// The id of a PureCloud External Organization entity in the External Contacts system that will be used to represent the trustor org
    #[serde(rename = "externalOrganizationId", skip_serializing_if = "Option::is_none")]
    pub external_organization_id: Option<String>,
    /// The id of a PureCloud organization that has granted trust to this PureCloud organization
    #[serde(rename = "trustorOrgId", skip_serializing_if = "Option::is_none")]
    pub trustor_org_id: Option<String>,
    /// Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The URI for the External Organization that is linked to the trustor org
    #[serde(rename = "externalOrganizationUri", skip_serializing_if = "Option::is_none")]
    pub external_organization_uri: Option<String>,
}

impl ExternalOrganizationTrustorLink {
    pub fn new() -> ExternalOrganizationTrustorLink {
        ExternalOrganizationTrustorLink {
            external_organization_id: None,
            trustor_org_id: None,
            date_created: None,
            external_organization_uri: None,
        }
    }
}

