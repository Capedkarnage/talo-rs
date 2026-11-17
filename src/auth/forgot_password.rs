use serde::Serialize;

use crate::{client::TaloClient, error::TaloResult};

#[derive(Serialize)]
struct ForgotPasswordRequest<'a> {
    #[serde(rename = "email")]
    email: &'a str,
}

impl TaloClient {
    pub async fn forgot_password(&self, email: &str) -> TaloResult<()> {
        let url = format!("{}/v1/players/auth/forgot_password", self.api_url);

        let payload = ForgotPasswordRequest { email };

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
