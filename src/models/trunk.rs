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
pub struct Trunk {
    /// The globally unique identifier for the object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the entity.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<Box<crate::models::Division>>,
    /// The resource's description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The current version of the resource.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The date the resource was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date of the last modification to the resource. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// The ID of the user that last modified the resource.
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// The ID of the user that created the resource.
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Indicates if the resource is active, inactive, or deleted.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The application that last modified the resource.
    #[serde(rename = "modifiedByApp", skip_serializing_if = "Option::is_none")]
    pub modified_by_app: Option<String>,
    /// The application that created the resource.
    #[serde(rename = "createdByApp", skip_serializing_if = "Option::is_none")]
    pub created_by_app: Option<String>,
    /// The type of this trunk.
    #[serde(rename = "trunkType", skip_serializing_if = "Option::is_none")]
    pub trunk_type: Option<TrunkType>,
    #[serde(rename = "edge", skip_serializing_if = "Option::is_none")]
    pub edge: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "trunkBase", skip_serializing_if = "Option::is_none")]
    pub trunk_base: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "trunkMetabase", skip_serializing_if = "Option::is_none")]
    pub trunk_metabase: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "edgeGroup", skip_serializing_if = "Option::is_none")]
    pub edge_group: Option<Box<crate::models::DomainEntityRef>>,
    /// True if this trunk is in-service.  This comes from the trunk_enabled property of the referenced trunk base.
    #[serde(rename = "inService", skip_serializing_if = "Option::is_none")]
    pub in_service: Option<bool>,
    /// True if the Edge used by this trunk is in-service
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "logicalInterface", skip_serializing_if = "Option::is_none")]
    pub logical_interface: Option<Box<crate::models::DomainEntityRef>>,
    #[serde(rename = "connectedStatus", skip_serializing_if = "Option::is_none")]
    pub connected_status: Option<Box<crate::models::TrunkConnectedStatus>>,
    /// The trunk optionsStatus
    #[serde(rename = "optionsStatus", skip_serializing_if = "Option::is_none")]
    pub options_status: Option<Vec<crate::models::TrunkMetricsOptions>>,
    /// The trunk registersStatus
    #[serde(rename = "registersStatus", skip_serializing_if = "Option::is_none")]
    pub registers_status: Option<Vec<crate::models::TrunkMetricsRegisters>>,
    #[serde(rename = "ipStatus", skip_serializing_if = "Option::is_none")]
    pub ip_status: Option<Box<crate::models::TrunkMetricsNetworkTypeIp>>,
    /// Returns Enabled when the trunk base supports the availability interval and it has a value greater than 0.
    #[serde(rename = "optionsEnabledStatus", skip_serializing_if = "Option::is_none")]
    pub options_enabled_status: Option<OptionsEnabledStatus>,
    /// Returns Enabled when the trunk base supports the registration interval and it has a value greater than 0.
    #[serde(rename = "registersEnabledStatus", skip_serializing_if = "Option::is_none")]
    pub registers_enabled_status: Option<RegistersEnabledStatus>,
    /// The IP Network Family of the trunk
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<i32>,
    /// The list of proxy addresses (ports if provided) for the trunk
    #[serde(rename = "proxyAddressList", skip_serializing_if = "Option::is_none")]
    pub proxy_address_list: Option<Vec<String>>,
    /// The URI for this object
    #[serde(rename = "selfUri", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,
}

impl Trunk {
    pub fn new(name: String) -> Trunk {
        Trunk {
            id: None,
            name,
            division: None,
            description: None,
            version: None,
            date_created: None,
            date_modified: None,
            modified_by: None,
            created_by: None,
            state: None,
            modified_by_app: None,
            created_by_app: None,
            trunk_type: None,
            edge: None,
            trunk_base: None,
            trunk_metabase: None,
            edge_group: None,
            in_service: None,
            enabled: None,
            logical_interface: None,
            connected_status: None,
            options_status: None,
            registers_status: None,
            ip_status: None,
            options_enabled_status: None,
            registers_enabled_status: None,
            family: None,
            proxy_address_list: None,
            self_uri: None,
        }
    }
}

/// Indicates if the resource is active, inactive, or deleted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}
/// The type of this trunk.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrunkType {
    #[serde(rename = "EXTERNAL")]
    EXTERNAL,
    #[serde(rename = "PHONE")]
    PHONE,
    #[serde(rename = "EDGE")]
    EDGE,
}

impl Default for TrunkType {
    fn default() -> TrunkType {
        Self::EXTERNAL
    }
}
/// Returns Enabled when the trunk base supports the availability interval and it has a value greater than 0.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OptionsEnabledStatus {
    #[serde(rename = "ENABLED")]
    ENABLED,
    #[serde(rename = "DISABLED")]
    DISABLED,
    #[serde(rename = "NOT_SUPPORTED")]
    NOTSUPPORTED,
}

impl Default for OptionsEnabledStatus {
    fn default() -> OptionsEnabledStatus {
        Self::ENABLED
    }
}
/// Returns Enabled when the trunk base supports the registration interval and it has a value greater than 0.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegistersEnabledStatus {
    #[serde(rename = "ENABLED")]
    ENABLED,
    #[serde(rename = "DISABLED")]
    DISABLED,
    #[serde(rename = "NOT_SUPPORTED")]
    NOTSUPPORTED,
}

impl Default for RegistersEnabledStatus {
    fn default() -> RegistersEnabledStatus {
        Self::ENABLED
    }
}

