use crate::workers::ws::WebSocket;

#[derive(Debug, Default)]
#[napi]
pub(crate) struct Client {
    ws: Option<WebSocket>,
    token: Option<String>
}

#[napi]
impl<'a> Client {
    /// Initialize a new Discord client
    #[napi(constructor)]
    pub fn new() -> Client {
        Client::default()
    }


}