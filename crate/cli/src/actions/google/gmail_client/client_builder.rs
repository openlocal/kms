use std::path::PathBuf;

use reqwest::{Client, Response};
use serde::Deserialize;

use super::{service_account::ServiceAccount, token::retrieve_token, GoogleApiError};
use crate::error::CliError;

#[derive(Deserialize)]
pub struct RequestError {
    pub error: ErrorContent,
}

#[derive(Deserialize)]
pub struct ErrorContent {
    pub message: String,
}

#[derive(Debug, Clone)]
struct GmailClientBuilder {
    service_account: ServiceAccount,
    user_id: String,
}

impl GmailClientBuilder {
    pub fn new(conf_path: &PathBuf, user_id: String) -> Result<Self, CliError> {
        let service_account = ServiceAccount::load_from_config(conf_path)?;
        Ok(Self {
            service_account,
            user_id,
        })
    }

    pub async fn build(self) -> Result<GmailClient, CliError> {
        let token = retrieve_token(&self.service_account, &self.user_id).await?;

        Ok(GmailClient {
            client: Client::new(),
            token,
            base_url: [
                "https://gmail.googleapis.com/gmail/v1/users/".to_string(),
                self.user_id,
            ]
            .concat(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GmailClient {
    client: Client,
    token: String,
    base_url: String,
}

impl GmailClient {
    pub async fn new(conf_path: &PathBuf, user_id: &str) -> Result<GmailClient, CliError> {
        GmailClientBuilder::new(conf_path, user_id.to_string())?
            .build()
            .await
    }

    pub async fn handle_response(response: Response) -> Result<(), CliError> {
        if response.status().is_success() {
            println!(
                "{}",
                response.text().await.map_err(GoogleApiError::Reqwest)?
            );
            Ok(())
        } else {
            let json_body = response
                .json::<RequestError>()
                .await
                .map_err(GoogleApiError::Reqwest)?;
            Err(CliError::GmailApiError(json_body.error.message.to_string()))
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Response, GoogleApiError> {
        self.client
            .get([&self.base_url, endpoint].concat())
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(GoogleApiError::from)
    }

    pub async fn post(&self, endpoint: &str, content: String) -> Result<Response, GoogleApiError> {
        self.client
            .post([&self.base_url, endpoint].concat())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::CONTENT_LENGTH, content.len())
            .body(content)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(GoogleApiError::from)
    }

    pub async fn patch(&self, endpoint: &str, content: String) -> Result<Response, GoogleApiError> {
        self.client
            .patch([&self.base_url, endpoint].concat())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::CONTENT_LENGTH, content.len())
            .body(content)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(GoogleApiError::from)
    }

    pub async fn delete(&self, endpoint: &str) -> Result<Response, GoogleApiError> {
        self.client
            .delete([&self.base_url, endpoint].concat())
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(GoogleApiError::from)
    }
}
