use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaloError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Player is not logged in")]
    NotAuthenticated,

    #[error("Player alias ID is missing")]
    MissingAliasId,

    #[error("Player ID is missing")]
    MissingPlayerId,

    #[error("Session token is missing")]
    MissingSessionToken,

    #[error("Email is required when verification is enabled")]
    EmailRequiredForVerification,
}

pub type TaloResult<T> = Result<T, TaloError>;
