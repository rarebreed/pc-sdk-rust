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
pub struct SiteConnection {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
    #[serde(rename = "managed", skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// Connection method from site to site (Direct, Indirect, CloudProxy
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Indicates if the current site is linked
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Media model for the current site.
    #[serde(rename = "mediaModel", skip_serializing_if = "Option::is_none")]
    pub media_model: Option<MediaModel>,
    /// All of the edges to which the site connects
    #[serde(rename = "edgeList", skip_serializing_if = "Option::is_none")]
    pub edge_list: Option<Vec<crate::models::ConnectedEdge>>,
    /// The core site
    #[serde(rename = "coreSite", skip_serializing_if = "Option::is_none")]
    pub core_site: Option<bool>,
    /// List of site ids names and selfUris for the primary core sites
    #[serde(rename = "primaryCoreSites", skip_serializing_if = "Option::is_none")]
    pub primary_core_sites: Option<Vec<crate::models::DomainEntityRef>>,
    /// List of site ids names and selfUris for the secondary core sites
    #[serde(rename = "secondaryCoreSites", skip_serializing_if = "Option::is_none")]
    pub secondary_core_sites: Option<Vec<crate::models::DomainEntityRef>>,
}

impl SiteConnection {
    pub fn new() -> SiteConnection {
        SiteConnection {
            id: None,
            name: None,
            self_uri: None,
            managed: None,
            _type: None,
            enabled: None,
            media_model: None,
            edge_list: None,
            core_site: None,
            primary_core_sites: None,
            secondary_core_sites: None,
        }
    }
}

/// Connection method from site to site (Direct, Indirect, CloudProxy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Direct")]
    Direct,
    #[serde(rename = "Indirect")]
    Indirect,
    #[serde(rename = "CloudProxy")]
    CloudProxy,
}

impl Default for Type {
    fn default() -> Type {
        Self::Direct
    }
}
/// Media model for the current site.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaModel {
    #[serde(rename = "Premises")]
    Premises,
    #[serde(rename = "Cloud")]
    Cloud,
}

impl Default for MediaModel {
    fn default() -> MediaModel {
        Self::Premises
    }
}
