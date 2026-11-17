use serde::Serialize;

use crate::{
    client::TaloClient,
    error::{TaloError, TaloResult},
    models::auth::AuthResponse,
};

#[derive(Serialize)]
struct VerifyRequest {
    #[serde(rename = "aliasId")]
    alias_id: u64,
    code: String,
}

impl TaloClient {
    pub async fn verify(&mut self, code: &str) -> TaloResult<AuthResponse> {
        let alias_id = self.alias_id.ok_or(TaloError::MissingAliasId)?;

        let url = format!("{}/v1/players/auth/verify", self.api_url);

        let payload = VerifyRequest {
            alias_id,
            code: code.to_string(),
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

        self.player_id = Some(response.alias.player.id.clone());
        self.alias_id = Some(response.alias.id);
        self.session_token = response.session_token.clone();
        self.socket_token = response.socket_token.clone();

        Ok(response)
    }
}
