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
pub struct AuditQueryFilter {
    /// Name of the property to filter.
    #[serde(rename = "property")]
    pub property: Property,
    /// Value of the property to filter.
    #[serde(rename = "value")]
    pub value: String,
}

impl AuditQueryFilter {
    pub fn new(property: Property, value: String) -> AuditQueryFilter {
        AuditQueryFilter {
            property,
            value,
        }
    }
}

/// Name of the property to filter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Property {
    #[serde(rename = "UserId")]
    UserId,
    #[serde(rename = "TrusteeOrganizationId")]
    TrusteeOrganizationId,
    #[serde(rename = "ClientId")]
    ClientId,
    #[serde(rename = "Action")]
    Action,
    #[serde(rename = "EntityType")]
    EntityType,
    #[serde(rename = "EntityId")]
    EntityId,
}

impl Default for Property {
    fn default() -> Property {
        Self::UserId
    }
}
