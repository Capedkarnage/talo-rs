use serde::Serialize;

use crate::{
    client::TaloClient,
    error::{TaloError, TaloResult},
    models::auth::AuthResponse,
};

#[derive(Serialize)]
struct MigrateAccountRequest<'a> {
    #[serde(rename = "currentPassword")]
    current_password: &'a str,

    identifier: &'a str,
    service: &'a str,
}

impl TaloClient {
    pub async fn migrate(
        &mut self,
        current_password: &str,
        identifier: &str,
        service: &str,
    ) -> TaloResult<AuthResponse> {
        let url = format!("{}/v1/players/auth/migrate", self.api_url);

        let alias_id = self.alias_id.ok_or(TaloError::MissingAliasId)?;

        let player_id = self
            .player_id
            .as_deref()
            .ok_or(TaloError::MissingPlayerId)?;

        let session_token = self
            .session_token
            .as_deref()
            .ok_or(TaloError::MissingSessionToken)?;

        let payload = MigrateAccountRequest {
            current_password,
            identifier,
            service,
        };

        let response = self
            .http
            .post(&url)
            .bearer_auth(&self.api_key)
            .header("x-talo-alias", alias_id)
            .header("x-talo-player", player_id)
            .header("x-talo-session", session_token)
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
        self.refresh_token = response.refresh_token.clone();

        Ok(response)
    }
}
