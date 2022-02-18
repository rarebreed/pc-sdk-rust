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
pub struct Adfs {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "issuerURI", skip_serializing_if = "Option::is_none")]
    pub issuer_uri: Option<String>,
    #[serde(rename = "ssoTargetURI", skip_serializing_if = "Option::is_none")]
    pub sso_target_uri: Option<String>,
    #[serde(rename = "sloURI", skip_serializing_if = "Option::is_none")]
    pub slo_uri: Option<String>,
    #[serde(rename = "sloBinding", skip_serializing_if = "Option::is_none")]
    pub slo_binding: Option<String>,
    #[serde(rename = "relyingPartyIdentifier", skip_serializing_if = "Option::is_none")]
    pub relying_party_identifier: Option<String>,
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "certificates", skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<String>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Adfs {
    pub fn new() -> Adfs {
        Adfs {
            id: None,
            name: None,
            disabled: None,
            issuer_uri: None,
            sso_target_uri: None,
            slo_uri: None,
            slo_binding: None,
            relying_party_identifier: None,
            certificate: None,
            certificates: None,
            self_uri: None,
        }
    }
}


