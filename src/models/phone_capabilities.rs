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
pub struct PhoneCapabilities {
    #[serde(rename = "provisions", skip_serializing_if = "Option::is_none")]
    pub provisions: Option<bool>,
    #[serde(rename = "registers", skip_serializing_if = "Option::is_none")]
    pub registers: Option<bool>,
    #[serde(rename = "dualRegisters", skip_serializing_if = "Option::is_none")]
    pub dual_registers: Option<bool>,
    #[serde(rename = "hardwareIdType", skip_serializing_if = "Option::is_none")]
    pub hardware_id_type: Option<String>,
    #[serde(rename = "allowReboot", skip_serializing_if = "Option::is_none")]
    pub allow_reboot: Option<bool>,
    #[serde(rename = "noRebalance", skip_serializing_if = "Option::is_none")]
    pub no_rebalance: Option<bool>,
    #[serde(rename = "noCloudProvisioning", skip_serializing_if = "Option::is_none")]
    pub no_cloud_provisioning: Option<bool>,
    #[serde(rename = "mediaCodecs", skip_serializing_if = "Option::is_none")]
    pub media_codecs: Option<Vec<MediaCodecs>>,
    #[serde(rename = "cdm", skip_serializing_if = "Option::is_none")]
    pub cdm: Option<bool>,
}

impl PhoneCapabilities {
    pub fn new() -> PhoneCapabilities {
        PhoneCapabilities {
            provisions: None,
            registers: None,
            dual_registers: None,
            hardware_id_type: None,
            allow_reboot: None,
            no_rebalance: None,
            no_cloud_provisioning: None,
            media_codecs: None,
            cdm: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaCodecs {
    #[serde(rename = "audio/opus")]
    Opus,
    #[serde(rename = "audio/pcmu")]
    Pcmu,
    #[serde(rename = "audio/pcma")]
    Pcma,
    #[serde(rename = "audio/g729")]
    G729,
    #[serde(rename = "audio/g722")]
    G722,
}

impl Default for MediaCodecs {
    fn default() -> MediaCodecs {
        Self::Opus
    }
}
