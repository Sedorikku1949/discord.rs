use std::fmt;
use crate::workers::ws::WebSocket;

#[derive(Debug, Default)]
#[napi]
pub(crate) struct Client {
    ws: Option<WebSocket>,
    pub token: Option<String>
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{ ws: {:?}, token: {:?} }}", self.ws, self.token)
    }
}

#[napi]
impl Client {
    /// Initialize a new Discord client
    #[napi(constructor)]
    pub fn new() -> Client {
        Client::default()
    }

    /// Login to Discord with a token
    /// Support only bot token
    /// Reference:
    /// https://discord.com/developers/docs/reference#authentication
    #[napi]
    pub async unsafe fn login(&mut self, token: String) {
        self.token = Some(token);
        self.ws = Some(WebSocket::init().await);
    }

    #[napi(getter)]
    pub fn token(&self) -> Option<String> {
        self.token.clone()
    }
}