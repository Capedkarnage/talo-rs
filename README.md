# talo-rs

An unofficial, asynchronous Rust client for the [Talo](https://trytalo.com) game backend API. The currently implemented API surface is player authentication.

## Features

<details>
<summary>Auth</summary>

- [x] `register(identifier, password, email)`
- [x] `login(identifier, password)`
- [x] `verify(code)`
- [x] `logout()`
- [x] `forgot_password(email)`
- [x] `reset_password(code, password)`
- [x] `change_password(current_password, new_password)`
- [x] `toggle_verification(current_password, enabled, email)`
- [x] `change_email(current_password, new_email)`
- [x] `change_identifier(current_password, new_identifier)`
- [x] `delete_account(current_password)`

</details>

## Requirements

- Rust 1.85 or later (Rust 2024 edition)
- A Talo API key
- A Tokio runtime

## Installation

Add the Git repository as a dependency:

```toml
[dependencies]
talo-rs = { git = "https://github.com/Capedkarnage/talo-rs" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Getting Started

Create a client with your Talo API key. Registration and login save the returned player, alias, and session details on the client for subsequent authenticated calls.

```rust
use talo_rs::TaloClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut talo = TaloClient::new("YOUR_TALO_API_KEY");

    let response = talo
        .register("player_username", "player_password", None)
        .await?;

    println!("Player ID: {}", response.alias.player.id);
    Ok(())
}
```

`register` accepts an optional email as its third argument. `login` accepts an identifier and password:

```rust
let response = talo.login("player_username", "player_password").await?;
println!("Logged in as {}", response.alias.identifier);
```

To use a different API URL, set it on the mutable client before making requests:

```rust
let mut talo = TaloClient::new("YOUR_TALO_API_KEY");
talo.with_api_url("https://api.trytalo.com");
```

`with_api_url` updates the client in place and does not return a client to chain.

## Authentication

`verify` uses the alias ID saved by a preceding registration or login and returns an `AuthResponse`:

```rust
let response = talo.verify("123456").await?;
println!("Verified player: {}", response.alias.player.id);
```

Account-management operations require a logged-in client. For example:

```rust
talo.change_password("current_password", "new_password").await?;
talo.change_email("current_password", "new@example.com").await?;
talo.change_identifier("current_password", "new_identifier").await?;
talo.toggle_verification("current_password", true, None).await?;
```

Password recovery does not require a logged-in session:

```rust
talo.forgot_password("player@example.com").await?;
talo.reset_password("123456", "new_password").await?;
```

Use `delete_account("current_password")` to delete the logged-in account. `logout()` clears the client’s saved player and session details.

## Error Handling

Client methods return `TaloResult<T>` (an alias for `Result<T, TaloError>`). HTTP failures and missing authentication state are reported as errors:

```rust
use talo_rs::{TaloClient, TaloError};

async fn login() -> Result<(), TaloError> {
    let mut talo = TaloClient::new("YOUR_TALO_API_KEY");
    let response = talo.login("player_username", "player_password").await?;
    println!("Welcome, {}!", response.alias.identifier);
    Ok(())
}
```

## Development

```bash
cargo test
cargo fmt --check
cargo clippy
```

## Contributing

Contributions, bug reports, and feature suggestions are welcome. Please open an issue or submit a pull request.

## Disclaimer

This is an unofficial, community-maintained Rust SDK. It is not affiliated with, endorsed by, or officially supported by Talo.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
