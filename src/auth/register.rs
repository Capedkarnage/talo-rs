use serde::Serialize;

use crate::{
    client::TaloClient,
    error::{TaloError, TaloResult},
    models::auth::AuthResponse,
};

#[derive(Serialize)]
struct RegisterRequest<'a> {
    identifier: &'a str,
    password: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,

    #[serde(rename = "verificationEnabled")]
    verification_enabled: bool,
}

impl TaloClient {
    pub async fn register(
        &mut self,
        username: &str,
        password: &str,
        email: Option<&str>,
        verification_enabled: bool,
    ) -> TaloResult<AuthResponse> {
        if verification_enabled && email.is_none_or(|email| email.trim().is_empty()) {
            return Err(TaloError::EmailRequiredForVerification);
        }

        let url = format!("{}/v1/players/auth/register", self.api_url);

        let payload = RegisterRequest {
            identifier: username,
            password,
            email,
            verification_enabled,
        };

        let response = self
            .http
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .json::<AuthResponse>()
            .await?;

        self.alias_id = Some(response.alias.id);
        self.player_id = Some(response.alias.player.id.clone());
        self.session_token = response.session_token.clone();
        self.socket_token = response.socket_token.clone();

        Ok(response)
    }
}
