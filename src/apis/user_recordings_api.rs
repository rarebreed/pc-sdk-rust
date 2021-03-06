/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`delete_userrecording`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserrecordingError {
    Status400(crate::models::ErrorBody),
    Status401(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status413(crate::models::ErrorBody),
    Status415(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    Status503(crate::models::ErrorBody),
    Status504(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_userrecording`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserrecordingError {
    Status400(crate::models::ErrorBody),
    Status401(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status413(crate::models::ErrorBody),
    Status415(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    Status503(crate::models::ErrorBody),
    Status504(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_userrecording_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserrecordingMediaError {
    Status400(crate::models::ErrorBody),
    Status401(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status413(crate::models::ErrorBody),
    Status415(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    Status503(crate::models::ErrorBody),
    Status504(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_userrecordings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserrecordingsError {
    Status400(crate::models::ErrorBody),
    Status401(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status413(crate::models::ErrorBody),
    Status415(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    Status503(crate::models::ErrorBody),
    Status504(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_userrecordings_summary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserrecordingsSummaryError {
    Status400(crate::models::ErrorBody),
    Status401(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status413(crate::models::ErrorBody),
    Status415(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    Status503(crate::models::ErrorBody),
    Status504(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_userrecording`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutUserrecordingError {
    Status400(crate::models::ErrorBody),
    Status401(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status413(crate::models::ErrorBody),
    Status415(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    Status503(crate::models::ErrorBody),
    Status504(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}


pub async fn delete_userrecording(configuration: &configuration::Configuration, recording_id: &str) -> Result<(), Error<DeleteUserrecordingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/userrecordings/{recordingId}", local_var_configuration.base_path, recordingId=crate::apis::urlencode(recording_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteUserrecordingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_userrecording(configuration: &configuration::Configuration, recording_id: &str, expand: Option<Vec<String>>) -> Result<crate::models::UserRecording, Error<GetUserrecordingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/userrecordings/{recordingId}", local_var_configuration.base_path, recordingId=crate::apis::urlencode(recording_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = expand {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("expand".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("expand", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserrecordingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_userrecording_media(configuration: &configuration::Configuration, recording_id: &str, format_id: Option<&str>) -> Result<crate::models::DownloadResponse, Error<GetUserrecordingMediaError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/userrecordings/{recordingId}/media", local_var_configuration.base_path, recordingId=crate::apis::urlencode(recording_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = format_id {
        local_var_req_builder = local_var_req_builder.query(&[("formatId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserrecordingMediaError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_userrecordings(configuration: &configuration::Configuration, page_size: Option<i32>, page_number: Option<i32>, expand: Option<Vec<String>>) -> Result<crate::models::UserRecordingEntityListing, Error<GetUserrecordingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/userrecordings", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_number {
        local_var_req_builder = local_var_req_builder.query(&[("pageNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("expand".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("expand", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserrecordingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_userrecordings_summary(configuration: &configuration::Configuration, ) -> Result<crate::models::FaxSummary, Error<GetUserrecordingsSummaryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/userrecordings/summary", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserrecordingsSummaryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn put_userrecording(configuration: &configuration::Configuration, recording_id: &str, body: crate::models::UserRecording, expand: Option<Vec<String>>) -> Result<crate::models::UserRecording, Error<PutUserrecordingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/userrecordings/{recordingId}", local_var_configuration.base_path, recordingId=crate::apis::urlencode(recording_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = expand {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("expand".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("expand", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutUserrecordingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

