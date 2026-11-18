# Notice
I do not reccomend using the SDK in its current state. It is currently lacking many features and Talo response handling. However, for this reason feel free to contribute!

# talo-rs

An unofficial, asynchronous Rust client for the [Talo](https://trytalo.com) game backend API.

## Talo REST API coverage

**Overall coverage: 20% (12 of 60 endpoints).**

<details>
<summary>Player authentication - 92% (12/13)</summary>

- [x] Register a player account
- [x] Log in to a player account
- [X] Refresh a player session
- [x] Verify a player account
- [x] Log out of a player account
- [x] Request a password reset
- [x] Reset a player password
- [x] Change a player password
- [x] Change a player email address
- [x] Change a player identifier
- [x] Enable or disable player verification
- [ ] Migrate a Talo player account to another service
- [x] Delete a player account

</details>

<details>
<summary>Players - 0% (0/6)</summary>

- [ ] Identify a player
- [ ] Search for players
- [ ] Find a player
- [ ] Merge two players
- [ ] Update a player's properties
- [ ] Create a player socket token

</details>

<details>
<summary>Player groups - 0% (0/1)</summary>

- [ ] Get a group and its members

</details>

<details>
<summary>Player relationships - 0% (0/6)</summary>

- [ ] List subscriptions from players
- [ ] List subscriptions for the current player
- [ ] Create a subscription request
- [ ] Confirm a pending subscription request
- [ ] Update a subscription's properties
- [ ] Delete a subscription

</details>

<details>
<summary>Player presence - 0% (0/2)</summary>

- [ ] Get a player's presence
- [ ] Update a player's online and custom status

</details>

<details>
<summary>Leaderboards - 0% (0/3)</summary>

- [ ] Get a leaderboard's entries
- [ ] Get top entries alongside the current player's entries
- [ ] Create or update a leaderboard entry

</details>

<details>
<summary>Game saves - 0% (0/4)</summary>

- [ ] Get a player's saves
- [ ] Create a save
- [ ] Update a save
- [ ] Delete a save

</details>

<details>
<summary>Game stats - 0% (0/7)</summary>

- [ ] Get all game stats
- [ ] Get all of a player's current stats
- [ ] Get a player's stat history
- [ ] Get a global stat's history
- [ ] Get a player's current value for a stat
- [ ] Get an individual game stat
- [ ] Update a stat value

</details>

<details>
<summary>Events - 0% (0/1)</summary>

- [ ] Track events

</details>

<details>
<summary>Feedback - 0% (0/2)</summary>

- [ ] Get available feedback categories
- [ ] Create a player feedback entry

</details>

<details>
<summary>Live config - 0% (0/1)</summary>

- [ ] Get the game's live config

</details>

<details>
<summary>Channels - 0% (0/13)</summary>

- [ ] List game channels
- [ ] List a player's channel subscriptions
- [ ] Get a channel's members
- [ ] Get multiple channel storage properties
- [ ] Get a channel storage property
- [ ] Find a game channel
- [ ] Create a game channel
- [ ] Join a game channel
- [ ] Invite a player to a game channel
- [ ] Leave a game channel
- [ ] Create or update channel storage properties
- [ ] Update a game channel
- [ ] Delete a game channel

</details>

<details>
<summary>Socket tickets - 0% (0/1)</summary>

- [ ] Create a socket ticket

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

## Usage
(Wiki to be made)

## Contributing

Contributions, bug reports, and feature suggestions are welcome. Please open an issue or submit a pull request.

## Disclaimer

This is an unofficial, community-maintained Rust SDK. It is not affiliated with, endorsed by, or officially supported by Talo.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
