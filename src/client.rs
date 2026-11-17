use reqwest::Client;


pub struct TaloClient {
    pub(crate) http: Client,
    pub(crate) api_key: String,
    pub(crate) api_url: String,

    pub player_id: Option<String>,
    pub alias_id: Option<u64>,
    pub session_token: Option<String>,
    pub socket_token: Option<String>,
}

impl TaloClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            api_key: api_key.into(),
            api_url: "https://api.trytalo.com".to_string(),

            player_id: None,
            alias_id: None,
            session_token: None,
            socket_token: None,
        }
    }

    pub fn with_api_url(&mut self, api_url: impl Into<String>) {
        self.api_url = api_url.into();
    }
}