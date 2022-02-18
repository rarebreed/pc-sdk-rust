use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod alerting_api;
pub mod analytics_api;
pub mod architect_api;
pub mod audit_api;
pub mod authorization_api;
pub mod billing_api;
pub mod chat_api;
pub mod coaching_api;
pub mod content_management_api;
pub mod conversations_api;
pub mod data_extensions_api;
pub mod external_contacts_api;
pub mod fax_api;
pub mod flows_api;
pub mod gamification_api;
pub mod general_data_protection_regulation_api;
pub mod geolocation_api;
pub mod greetings_api;
pub mod groups_api;
pub mod identity_provider_api;
pub mod integrations_api;
pub mod journey_api;
pub mod knowledge_api;
pub mod language_understanding_api;
pub mod languages_api;
pub mod learning_api;
pub mod license_api;
pub mod locations_api;
pub mod messaging_api;
pub mod mobile_devices_api;
pub mod notifications_api;
pub mod o_auth_api;
pub mod objects_api;
pub mod organization_api;
pub mod organization_authorization_api;
pub mod outbound_api;
pub mod presence_api;
pub mod quality_api;
pub mod recording_api;
pub mod response_management_api;
pub mod routing_api;
pub mod scim_api;
pub mod scripts_api;
pub mod search_api;
pub mod speech_text_analytics_api;
pub mod stations_api;
pub mod suggest_api;
pub mod telephony_api;
pub mod telephony_providers_edge_api;
pub mod textbots_api;
pub mod tokens_api;
pub mod uploads_api;
pub mod usage_api;
pub mod user_recordings_api;
pub mod users_api;
pub mod utilities_api;
pub mod voicemail_api;
pub mod web_chat_api;
pub mod web_deployments_api;
pub mod web_messaging_api;
pub mod widgets_api;
pub mod workforce_management_api;

pub mod configuration;
