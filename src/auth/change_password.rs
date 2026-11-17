use serde::Serialize;

use crate::{
    client::TaloClient,
    error::{TaloError, TaloResult},
};


#[derive(Serialize)]
struct ChangePasswordRequest<'a> {
    #[serde(rename = "currentPassword")]
    current_password: &'a str,

    #[serde(rename = "newPassword")]
    new_password: &'a str,
}

impl TaloClient {
    pub async fn change_password(
        &self,
        current_password: &str,
        new_password: &str,
    ) -> TaloResult<()> {
        let url = format!(
            "{}/v1/players/auth/change_password",
            self.api_url
        );

        let alias_id = self.alias_id
            .ok_or(TaloError::MissingAliasId)?;

        let player_id = self.player_id
            .as_deref()
            .ok_or(TaloError::MissingPlayerId)?;

        let session_token = self.session_token
            .as_deref()
            .ok_or(TaloError::MissingSessionToken)?;

        let payload = ChangePasswordRequest {
            current_password,
            new_password,
        };

        self.http
            .post(&url)
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
