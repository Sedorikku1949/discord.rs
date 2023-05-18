use std::net::TcpStream;
use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use url::Url;
use crate::constants::GATEWAY_URL;

#[derive(Debug)]
#[napi]
pub struct WebSocket {
    instance: tungstenite::WebSocket<MaybeTlsStream<TcpStream>>
}

impl WebSocket {
    /// Initialize a new WebSocket connection to the Discord gateway
    /// Will panic if the connection fails
    /// Reference:
    /// https://discord.com/developers/docs/topics/gateway#connecting-to-the-gateway
    pub fn init() -> Self {
        let gateway_url = if let Ok(u) = Url::parse(GATEWAY_URL) { u }
        else { panic!("Invalid gateway URL") };
        let (socket, response) = connect(gateway_url).expect("Failed to connect to the gateway");

        if !response.status().is_success() {
            panic!("Failed to connect to the gateway. Status code: {}", response.status());
        };

        Self {
            instance: socket
        }
    }
}