
# talo-rs

An unofficial Rust SDK for the [Talo](https://trytalo.com) game backend API.

`talo-rs` provides an asynchronous Rust interface for interacting with Talo, including player authentication and session management.

## Features

- [x] Player login
- [x] Player registration
- [x] Player logout
- [x] Player verification
- [x] Change password
- [x] Toggle account verification
- [ ] Player data
- [ ] Leaderboards
- [ ] Statistics
- [ ] WebSocket events

## Requirements

- Rust 1.75 or later
- A Talo API key

## Installation

Add `talo-rs` to your `Cargo.toml`:

```toml
[dependencies]
talo-rs = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

If the crate is not published on crates.io yet, use its Git repository instead:

```toml
[dependencies]
talo-rs = { git = "https://github.com/YOUR_USERNAME/talo-rs" }
```

## Getting Started

Create a client using your Talo API key:

```rust
use talo_rs::TaloClient;

#[tokio::main]
async fn main() {
    let mut talo = TaloClient::new(
        "YOUR_TALO_API_KEY",
        None::<String>,
    );
}
```

You can also provide a custom API URL:

```rust
let mut talo = TaloClient::new(
    "YOUR_TALO_API_KEY",
    Some("https://api.trytalo.com"),
);
```

## Authentication

### Register

```rust
let response = talo
    .register("player_username", "player_password", None)
    .await?;

println!("Player ID: {}", response.alias.player.id);
```

### Login

```rust
let response = talo
    .login("player_username", "player_password")
    .await?;

println!("Logged in!");
println!("Player ID: {}", response.alias.player.id);
```

### Verify Account

```rust
let response = talo.verify("123456").await?;

println!("Account verified!");
```

### Change Password

```rust
talo.change_password(
    "current_password",
    "new_password",
).await?;

println!("Password changed!");
```

### Toggle Verification

```rust
talo.toggle_verification(true).await?;

println!("Verification settings updated!");
```

### Logout

```rust
talo.logout().await?;

println!("Logged out!");
```

## Error Handling

All SDK methods return a `Result`, allowing you to handle API and authentication errors:

```rust
use talo_rs::{TaloClient, TaloError};

match talo.login("username", "password").await {
    Ok(response) => {
        println!("Welcome, {}!", response.alias.identifier);
    }
    Err(error) => {
        eprintln!("Login failed: {}", error);
    }
}
```

## Async Runtime

`talo-rs` uses Tokio and asynchronous HTTP requests. Make sure your application runs inside a Tokio runtime.

## Development

Clone the repository:

```bash
git clone https://github.com/Capedkarange/talo-rs.git
cd talo-rs
```

Run the tests:

```bash
cargo test
```

Format the code:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy
```

## Contributing

Contributions, bug reports, and feature suggestions are welcome!

Please open an issue or submit a pull request.

## Disclaimer

This is an unofficial, community-maintained Rust SDK. It is not affiliated with, endorsed by, or officially supported by Talo.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
