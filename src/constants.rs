#[napi]
pub const API_VERSION: &str = "10";
#[napi]
pub const API_BASE_URL: &str = "https://discord.com/api/v10/";

pub const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub const USER_AGENT: &str = concat!("DiscordBot (", env!("CARGO_PKG_HOMEPAGE"), ", ", env!("CARGO_PKG_VERSION"), ")");