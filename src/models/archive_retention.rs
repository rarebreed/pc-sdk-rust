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
pub struct ArchiveRetention {
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "storageMedium", skip_serializing_if = "Option::is_none")]
    pub storage_medium: Option<StorageMedium>,
}

impl ArchiveRetention {
    pub fn new() -> ArchiveRetention {
        ArchiveRetention {
            days: None,
            storage_medium: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StorageMedium {
    #[serde(rename = "CLOUDARCHIVE")]
    CLOUDARCHIVE,
}

impl Default for StorageMedium {
    fn default() -> StorageMedium {
        Self::CLOUDARCHIVE
    }
}

