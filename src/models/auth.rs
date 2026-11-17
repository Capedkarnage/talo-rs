use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub alias: PlayerAlias,

    #[serde(rename = "sessionToken")]
    pub session_token: Option<String>,

    #[serde(rename = "socketToken")]
    pub socket_token: Option<String>,

    #[serde(rename = "verificationRequired")]
    pub verification_required: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerAlias {
    pub id: u64,
    pub identifier: String,
    pub player: PlayerData,
}

#[derive(Debug, Deserialize)]
pub struct PlayerData {
    pub id: String,
}
