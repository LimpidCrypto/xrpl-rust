use xrpl::asynch::clients::{Message, WebsocketIO};
use xrpl::models::requests::AccountInfo;
use crate::common::{connect_to_ws_echo, connect_to_wss_echo};

#[tokio::test]
async fn test_websocket_non_tls() {
    let mut websocket = connect_to_ws_echo().await;
    let account_info = AccountInfo::new("rogue5HnPRSszD9CWGSUz8UGHMVwSSKF6", None, None, None, None, None, None);
    websocket.send(account_info.clone()).await.unwrap();

    loop {
        let message = websocket.on_message().await.unwrap().unwrap();
        match message {
            Message::Text(text) => {
                let account_info_as_string = serde_json::to_string(&account_info.clone()).unwrap();
                println!("{:?}", text);
                assert_eq!(account_info_as_string, text)
            }
            _ => panic!("Expected account_info json as text message.")
        }
        break;
    }
}

#[tokio::test]
async fn test_websocket_tls() {
    let mut websocket = connect_to_wss_echo().await;
    let account_info = AccountInfo::new("rogue5HnPRSszD9CWGSUz8UGHMVwSSKF6", None, None, None, None, None, None);
    websocket.send(account_info.clone()).await.unwrap();

    loop {
        let message = websocket.on_message().await.unwrap().unwrap();
        match message {
            Message::Text(text) => {
                let account_info_as_string = serde_json::to_string(&account_info.clone()).unwrap();
                println!("{:?}", text);
                assert_eq!(account_info_as_string, text)
            }
            _ => panic!("Expected account_info json as text message.")
        }
        break;
    }
}
