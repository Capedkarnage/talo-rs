pub mod client;
pub mod auth;
pub mod models;
pub mod error;

pub use client::TaloClient;
pub use error::{TaloError, TaloResult};
pub use models::auth::{AuthResponse, PlayerAlias, PlayerData};
