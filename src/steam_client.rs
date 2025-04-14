use std::fmt;

use reqwest::{Client, Method, StatusCode, Url};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[allow(dead_code)]
const DEFAULT_BASE_URL: &str = "http://api.steampowered.com";

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum SteamClientError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Bad Request: Invalid request parameters")]
    BadRequest,
    #[error("Unauthorized: Authentication required")]
    Unauthorized,
    #[error("Forbidden: Access denied")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Unprocessable Entity: Server cannot process the request")]
    UnprocessableEntity,
    #[error("Too Many Requests: Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Internal Server Error: Unexpected error occurred")]
    InternalServerError,
    #[error("API error: {status}, message: {message}")]
    ApiError { status: StatusCode, message: String },
    #[error("Unexpected error: {0}")]
    Other(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Invalid url: {0}")]
    InvalidUrl(String),
}

pub enum RelationshipType {
    ALL,
    FRIEND
}

impl fmt::Display for RelationshipType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RelationshipType::ALL => write!(f, "all"),
            RelationshipType::FRIEND => write!(f, "friend")
        }
    }
}

#[allow(dead_code)]
pub struct SteamClient {
    client: Client,
    api_key: String,
    base_url: Url
}

impl SteamClient {
    pub fn new(api_key: String) -> Self {
        SteamClient {
            client: Client::new(),
            api_key: api_key,
            base_url: Url::parse(DEFAULT_BASE_URL).expect("base_url must be a valid URL")
        }
    }

    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str
    ) -> Result<T, SteamClientError> {
        let url = self.base_url.join(path).expect("path must be a valid URL");
        let request = self.client.request(Method::GET, url);

        let response = request.send().await?;

        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
                Ok(response.json().await?)
            }
            StatusCode::BAD_REQUEST => Err(SteamClientError::BadRequest),
            StatusCode::UNAUTHORIZED => Err(SteamClientError::Unauthorized),
            StatusCode::FORBIDDEN => Err(SteamClientError::Forbidden),
            StatusCode::NOT_FOUND => Err(SteamClientError::NotFound),
            StatusCode::UNPROCESSABLE_ENTITY => Err(SteamClientError::UnprocessableEntity),
            StatusCode::TOO_MANY_REQUESTS => Err(SteamClientError::RateLimitExceeded),
            StatusCode::INTERNAL_SERVER_ERROR => Err(SteamClientError::InternalServerError),
            status => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(SteamClientError::ApiError { status, message })
            }
        }
    }

    pub async fn get_news_for_app(
        &self, 
        appid: i64, 
        count: i64, 
        maxlength: i64
    ) -> Result<AppNewsResponse, SteamClientError> {
        let path = format!("/ISteamNews/GetNewsForApp/v0002/?appid={appid}&count={count}&maxlength={maxlength}");
        self.request(&path).await
    }

    pub async fn get_global_achievement_percentages_for_app(
        &self, 
        gameid: i64
    ) -> Result<GlobalAchievementResponse, SteamClientError> {
        let path = format!("/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/?gameid={gameid}");
        self.request(&path).await
    }

    pub async fn get_player_summaries(
        &self,
        steamids: Vec<String>
    ) -> Result<PlayerSummariesResponse, SteamClientError> {
        let path = format!("/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}", self.api_key, steamids.join(","));
        self.request(&path).await
    }

    pub async fn get_friend_list(
        &self,
        steamid: String,
        relationship: RelationshipType
    ) -> Result<FriendsListResponse, SteamClientError> {
        let path = format!("/ISteamUser/GetFriendList/v0001/?key={}&steamid={steamid}&relationship={relationship}", self.api_key);
        self.request(&path).await
    }

    pub async fn get_player_achievements(
        &self,
        appid: i64,
        steamid: String
    ) -> Result<PlayerAchievementsResponse, SteamClientError> {
        let path = format!("/ISteamUserStats/GetPlayerAchievements/v0001/?appid={appid}&key={}&steamid={steamid}", self.api_key);
        self.request(&path).await
    }

    pub async fn get_user_stats_for_game(
        &self,
        appid: i64,
        steamid: String
    ) -> Result<PlayerStatsResponse, SteamClientError> {
        let path = format!("/ISteamUserStats/GetUserStatsForGame/v0002/?appid={appid}&key={}&steamid={steamid}", self.api_key);
        self.request(&path).await
    }

    pub async fn get_owned_games(
        &self,
        steamid: String,
        include_appinfo: bool,
        include_played_free_games: bool
    ) -> Result<OwnedGamesResponse, SteamClientError> {
        let path = format!("/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={steamid}&include_appinfo={include_appinfo}&include_played_free_games={include_played_free_games}", self.api_key);
        self.request(&path).await
    }

    pub async fn get_recently_played_games(
        &self,
        steamid: String,
        count: i64
    ) -> Result<RecentlyPlayedGamesResponse, SteamClientError> {
        let path = format!("/IPlayerService/GetRecentlyPlayedGames/v0001/?key={}&steamid={steamid}&count={count}", self.api_key);
        self.request(&path).await
    }

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppNewsResponse {
    pub appnews: AppNews,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppNews {
    pub appid: i64,
    pub newsitems: Vec<NewsItem>,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsItem {
    pub gid: String,
    pub title: String,
    pub url: String,
    #[serde(rename = "is_external_url")]
    pub is_external_url: bool,
    pub author: String,
    pub contents: String,
    pub feedlabel: String,
    pub date: i64,
    pub feedname: String,
    #[serde(rename = "feed_type")]
    pub feed_type: i64,
    pub appid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalAchievementResponse {
    pub achievementpercentages: GlobalAchievementPercentages,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalAchievementPercentages {
    pub achievements: Vec<AchievementPercentage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementPercentage {
    pub name: String,
    pub percent: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSummariesResponse {
    #[serde(rename="response")]
    pub playersummaries: PlayerSummaries,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSummaries {
    pub players: Vec<Player>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub steamid: String,
    pub communityvisibilitystate: i64,
    pub profilestate: i64,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub avatarhash: String,
    pub personastate: i64,
    pub realname: Option<String>,
    pub primaryclanid: Option<String>,
    pub timecreated: Option<i64>,
    pub personastateflags: Option<i64>,
    pub loccountrycode: Option<String>,
    pub locstatecode: Option<String>,
    pub loccityid: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendsListResponse {
    pub friendslist: FriendsList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendsList {
    pub friends: Vec<Friend>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Friend {
    pub steamid: String,
    pub relationship: String,
    #[serde(rename = "friend_since")]
    pub friend_since: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAchievementsResponse {
    pub playerstats: PlayerAchievements,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAchievements {
    #[serde(rename = "steamID")]
    pub steam_id: String,
    pub game_name: String,
    pub achievements: Vec<Achievement>,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub apiname: String,
    pub achieved: i64,
    pub unlocktime: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatsResponse {
    pub playerstats: PlayerStats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    #[serde(rename = "steamID")]
    pub steam_id: String,
    pub game_name: String,
    pub stats: Vec<Stat>,
    pub achievements: Vec<PlayerStatsAchievement>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub name: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatsAchievement {
    pub name: String,
    pub achieved: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedGamesResponse {
    pub response: OwnedGamesData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedGamesData {
    #[serde(rename = "game_count")]
    pub game_count: i64,
    pub games: Vec<OwnedGame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedGame {
    pub appid: i64,
    pub name: Option<String>,
    #[serde(rename = "playtime_forever")]
    pub playtime_forever: i64,
    #[serde(rename = "img_icon_url")]
    pub img_icon_url: Option<String>,
    #[serde(rename = "content_descriptorids")]
    #[serde(default)]
    pub content_descriptorids: Vec<i64>,
    #[serde(rename = "has_community_visible_stats")]
    pub has_community_visible_stats: Option<bool>,
    #[serde(rename = "has_leaderboards")]
    pub has_leaderboards: Option<bool>,
    #[serde(rename = "playtime_2weeks")]
    pub playtime_2weeks: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyPlayedGamesResponse {
    #[serde(rename = "response")]
    pub recently_played_games: RecentlyPlayedGamesData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyPlayedGamesData {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    pub games: Vec<RecentlyPlayedGame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentlyPlayedGame {
    pub appid: i64,
    pub name: String,
    #[serde(rename = "playtime_2weeks")]
    pub playtime_2weeks: i64,
    #[serde(rename = "playtime_forever")]
    pub playtime_forever: i64,
    #[serde(rename = "img_icon_url")]
    pub img_icon_url: String,
}