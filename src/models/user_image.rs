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
pub struct UserImage {
    /// Height and/or width of image. ex: 640x480 or x128
    #[serde(rename = "resolution", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "imageUri", skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
}

impl UserImage {
    pub fn new() -> UserImage {
        UserImage {
            resolution: None,
            image_uri: None,
        }
    }
}


