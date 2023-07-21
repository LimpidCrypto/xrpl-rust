mod constants;
pub use constants::*;
use xrpl::asynch::clients::{AsyncWebsocketClientTokio, Open, WebsocketOpen};

pub async fn connect_to_ws_echo<'a>() -> AsyncWebsocketClientTokio<Open> {
    let websocket = AsyncWebsocketClientTokio::open(ECHO_WS_SERVER.parse().unwrap()).await.unwrap();
    assert!(websocket.is_open());

    websocket
}

pub async fn connect_to_wss_echo<'a>() -> AsyncWebsocketClientTokio<Open> {
    let websocket = AsyncWebsocketClientTokio::open(ECHO_WSS_SERVER.parse().unwrap()).await.unwrap();
    assert!(websocket.is_open());

    websocket
}
