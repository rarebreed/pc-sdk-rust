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
pub struct GeneralProgramJobRequest {
    /// The dialect of the topics to link with the general program, dialect format is {language}-{country} where language follows ISO 639-1 standard and country follows ISO 3166-1 alpha 2 standard
    #[serde(rename = "dialect")]
    pub dialect: Dialect,
    /// The mode to use for the general program job, default value is Skip
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
}

impl GeneralProgramJobRequest {
    pub fn new(dialect: Dialect) -> GeneralProgramJobRequest {
        GeneralProgramJobRequest {
            dialect,
            mode: None,
        }
    }
}

/// The dialect of the topics to link with the general program, dialect format is {language}-{country} where language follows ISO 639-1 standard and country follows ISO 3166-1 alpha 2 standard
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Dialect {
    #[serde(rename = "en-US")]
    EnUS,
    #[serde(rename = "es-US")]
    EsUS,
    #[serde(rename = "en-AU")]
    EnAU,
    #[serde(rename = "en-GB")]
    EnGB,
    #[serde(rename = "en-ZA")]
    EnZA,
    #[serde(rename = "es-ES")]
    EsES,
    #[serde(rename = "en-IN")]
    EnIN,
    #[serde(rename = "fr-FR")]
    FrFR,
    #[serde(rename = "fr-CA")]
    FrCA,
    #[serde(rename = "it-IT")]
    ItIT,
    #[serde(rename = "de-DE")]
    DeDE,
    #[serde(rename = "pt-BR")]
    PtBR,
    #[serde(rename = "pl-PL")]
    PlPL,
}

impl Default for Dialect {
    fn default() -> Dialect {
        Self::EnUS
    }
}
/// The mode to use for the general program job, default value is Skip
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "Skip")]
    Skip,
    #[serde(rename = "Merge")]
    Merge,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Skip
    }
}

