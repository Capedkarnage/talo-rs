use serde::Serialize;

use crate::{
    client::TaloClient,
    error::TaloResult,
};


#[derive(Serialize)]
struct ResetPasswordRequest<'a> {
    #[serde(rename = "email")]
    code: &'a str,
    
    #[serde(rename = "password")]
    password: &'a str,
}

impl TaloClient {
    pub async fn reset_password(
        &self,
        code: &str,
        password: &str,
    ) -> TaloResult<()> {
        let url = format!(
            "{}/v1/players/auth/reset_password",
            self.api_url
        );

        let payload = ResetPasswordRequest {
            code,
            password,
        };

        self.http
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
