use serde::Serialize;

use crate::{client::TaloClient, error::TaloResult, models::auth::AuthResponse};

#[derive(Serialize)]
struct RefreshRequest<'a> {
    #[serde(rename = "refreshToken")]
    refresh_token: &'a str,
}

impl TaloClient {
    pub async fn refresh(
        &mut self,
        refresh_token: &str,
    ) -> TaloResult<AuthResponse> {
        let url = format!("{}/v1/players/auth/refresh", self.api_url);

        let payload = RefreshRequest { refresh_token };

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

        self.session_token = response.session_token.clone();
        self.refresh_token = response.refresh_token.clone();

        Ok(response)
    }
}
