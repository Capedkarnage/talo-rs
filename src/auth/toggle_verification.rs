use crate::{
    client::TaloClient,
    error::{TaloError, TaloResult},
};
use serde::Serialize;

#[derive(Serialize)]
struct ToggleVerificationRequest<'a> {
    #[serde(rename = "currentPassword")]
    current_password: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,

    #[serde(rename = "verificationEnabled")]
    verification_enabled: bool,
}

impl TaloClient {
    pub async fn toggle_verification(
        &self,
        current_password: &str,
        enabled: bool,
        email: Option<&str>,
    ) -> TaloResult<()> {
        let alias_id = self.alias_id.ok_or(TaloError::MissingAliasId)?;
        let player_id = self
            .player_id
            .as_deref()
            .ok_or(TaloError::MissingPlayerId)?;
        let session_token = self
            .session_token
            .as_deref()
            .ok_or(TaloError::MissingSessionToken)?;

        let url = format!("{}/v1/players/auth/toggle_verification", self.api_url);

        let payload = ToggleVerificationRequest {
            current_password,
            email,
            verification_enabled: enabled,
        };

        self.http
            .patch(&url)
            .bearer_auth(&self.api_key)
            .header("x-talo-alias", alias_id)
            .header("x-talo-player", player_id)
            .header("x-talo-session", session_token)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
