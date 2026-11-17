use crate::{
    client::TaloClient,
    error::TaloResult,
};


impl TaloClient {
    pub async fn logout(&mut self) -> TaloResult<()> {
        if let Some(token) = &self.session_token {
            let url = format!("{}/v1/players/auth/logout", self.api_url);

            self.http
                .post(&url)
                .bearer_auth(&self.api_key)
                .header("X-Talo-Session", token)
                .send()
                .await?
                .error_for_status()?;
        }

        self.player_id = None;
        self.alias_id = None;
        self.session_token = None;
        self.socket_token = None;

        Ok(())
    }
}
