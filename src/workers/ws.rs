use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use url::Url;
use crate::constants::{API_BASE_URL, GATEWAY_URL};

#[derive(Debug)]
#[napi]
pub struct WebSocket {
    instance: Arc<Mutex<tungstenite::WebSocket<MaybeTlsStream<TcpStream>>>>,
    receive_thread: Option<JoinHandle<()>>,

    pub gateway_url: String,
}

impl WebSocket {

    /// Retrieve the gateway URL from the Discord API
    /// Will panic if the request fails
    async fn retrieve_gateway_url() -> String {
        let res = reqwest::get(format!("{}gateway", API_BASE_URL)).await.unwrap();

        let body = res.json::<serde_json::Value>().await.expect("Failed to parse JSON from gateway URL response");

        format!(
            "{}/?v=10&encoding=json",
            if let Some(url) = body["url"].as_str() {
                String::from(url)
            } else {
                String::from(GATEWAY_URL)
            }
        )
    }

    /// Initialize a new WebSocket connection to the Discord gateway
    /// Will panic if the connection fails
    /// Reference:
    /// https://discord.com/developers/docs/topics/gateway#connecting-to-the-gateway
    pub async fn init() -> Self {
        let gateway = Self::retrieve_gateway_url().await;

        dbg!(&gateway);

        let gateway_url = if let Ok(u) = Url::parse(gateway.clone().as_str()) { u }
        else { panic!("Invalid gateway URL") };

        let (socket, response) = match connect(gateway_url) {
            Ok((socket, response)) => (socket, response),
            Err(e) => {
                // send body for debug
                let body = e.to_string();
                dbg!(body);
                panic!("Failed to connect to the gateway. Error: {}", e);
            },
        };

        if !response.status().is_success() {
            panic!("Failed to connect to the gateway. Status code: {}", response.status());
        };

        let socket_mutex = Arc::new(Mutex::new(socket));


        let socket_mutex_clone = socket_mutex.clone();
        let receive_thread = Some(thread::spawn(move || {
            loop {
                let mut socket = socket_mutex_clone.lock().unwrap();
                let message = socket.read_message().expect("Failed to read message from the gateway");
                dbg!("Received message from gateway");
                // Gérer le message ici
                WebSocket::receiver_handler(message.to_text().unwrap().to_string());
            }
        }));

        Self {
            instance: socket_mutex,
            receive_thread,
            gateway_url: gateway,
        }
    }

    /// Send a message to the gateway
    pub fn send(&mut self, message: String) {
        let mut socket = self.instance.lock().unwrap();
        socket.write_message(tungstenite::Message::Text(message)).unwrap();
    }

    /// Receive a message from the gateway
    pub fn receive(&mut self) -> String {
        let mut socket = self.instance.lock().unwrap();
        socket.read_message().unwrap().to_text().unwrap().to_string()
    }

    fn receiver_handler(message: String) {
        println!("Received message: {:?}", message);
    }
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        let mut socket = self.instance.lock().unwrap();
        // Fermer la connexion WebSocket avant de terminer la thread
        socket.close(None).unwrap();
        // Attendre que la thread de réception se termine
        if let Some(receive_thread) = self.receive_thread.take() {
            receive_thread.join().unwrap();
        }
    }
}