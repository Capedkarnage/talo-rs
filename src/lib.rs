pub mod auth;
pub mod client;
pub mod error;
pub mod models;

pub use client::TaloClient;
pub use error::{TaloError, TaloResult};
pub use models::auth::{AuthResponse, PlayerAlias, PlayerData};
