use crate::{client::TaloClient, error::TaloResult, models::auth::AuthResponse};

use serde::Serialize;

#[derive(Serialize)]
struct LoginRequest<'a> {
    identifier: &'a str,
    password: &'a str,
}

impl TaloClient {
    pub async fn login(&mut self, identifier: &str, password: &str) -> TaloResult<AuthResponse> {
        let url = format!("{}/v1/players/auth/login", self.api_url);

        let payload = LoginRequest {
            identifier,
            password,
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
