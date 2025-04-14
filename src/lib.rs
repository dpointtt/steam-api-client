mod steam_client;

pub use steam_client::SteamClient;
pub use steam_client::SteamClientError;

pub use steam_client::{
    AppNewsResponse,
    AppNews,
    NewsItem,
    GlobalAchievementResponse,
    GlobalAchievementPercentages,
    AchievementPercentage,
    PlayerSummariesResponse,
    PlayerSummaries,
    Player,
    FriendsListResponse,
    FriendsList,
    Friend,
    PlayerAchievementsResponse,
    PlayerAchievements,
    Achievement,
    PlayerStatsResponse,
    PlayerStats,
    Stat,
    PlayerStatsAchievement,
    OwnedGamesResponse,
    OwnedGamesData,
    OwnedGame,
    RecentlyPlayedGamesResponse,
    RecentlyPlayedGamesData,
    RecentlyPlayedGame,
};