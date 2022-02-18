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
pub struct NuanceMixDlgSettings {
    /// The Nuance channel ID to use when launching the Nuance bot, which must one of the code names of the bot's registered input channels.
    #[serde(rename = "channelId", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Name/value pairs of input variables to be sent to the Nuance bot. The values must be in the appropriate format for the variable's type (see https://docs.mix.nuance.com/dialog-grpc/v1/#simple-variable-types for help)
    #[serde(rename = "inputParameters", skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl NuanceMixDlgSettings {
    pub fn new() -> NuanceMixDlgSettings {
        NuanceMixDlgSettings {
            channel_id: None,
            input_parameters: None,
        }
    }
}


