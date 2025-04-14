# Steam API Client for Rust

A Rust library for interacting with the Steam Web API. This client provides easy-to-use async methods for accessing various Steam services and data.

## Features

- Fully async implementation using Tokio and Reqwest
- Strongly typed response structures with Serde serialization/deserialization
- Comprehensive error handling
- Support for many Steam API endpoints including:
  - News for games
  - Global achievement statistics
  - Player summaries and profiles
  - Friend lists
  - Player achievements
  - User game stats
  - Owned games
  - Recently played games

## Installation

```sh
cargo add steam-api-client
```

## Usage

```rust
use steam_api_client::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Init steam client
    let client = SteamClient::new("STEAM_WEB_API_KEY".to_string());

    let steam_ids = vec![
        "76561198316852340".to_string(),
    ];

    // Fetch players summaries
    let response: PlayerSummariesResponse = client.get_player_summaries(steam_ids).await?;

    // You also can print data with debug
    println!("{:?}", response);

    // Extracting data
    let players: Vec<Player> = response.playersummaries.players;
    for player in players {
        println!("Player profile url: {}", player.profileurl)
    }

    Ok(())

}
```

## API Reference

### SteamClient Methods

| Method | Description |
|--------|-------------|
| `get_news_for_app(appid: i64, count: i64, maxlength: i64)` | Gets news for the specified app |
| `get_global_achievement_percentages_for_app(gameid: i64)` | Gets global achievement percentages |
| `get_player_summaries(steamids: Vec<String>)` | Gets player summaries for the specified Steam IDs |
| `get_friend_list(steamid: String, relationship: String)` | Gets a user's friend list |
| `get_player_achievements(appid: i64, steamid: String)` | Gets achievements for a player in a specific game |
| `get_user_stats_for_game(appid: i64, steamid: String)` | Gets user stats for a specific game |
| `get_owned_games(steamid: String, include_appinfo: bool, include_played_free_games: bool)` | Gets a user's owned games |
| `get_recently_played_games(steamid: String, count: i64)` | Gets a user's recently played games |

## Requirements

- A valid Steam API key for some of the requests (obtain one at [https://steamcommunity.com/dev/apikey](https://steamcommunity.com/dev/apikey))

## License

This project is licensed under the MIT License - see the LICENSE file for details.