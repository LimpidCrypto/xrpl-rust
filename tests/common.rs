use std::borrow::Cow;
use xrpl::asynch::clients::{AsyncWebsocketClient, ReadResult, Websocket};
use xrpl::models::requests::AccountInfo;

#[tokio::test]
async fn test_websocket() {
    let mut buffer = [0u8; 4096];
    let uri = Cow::from("ws://limpidcrypto.de:6004");
    let mut ws = AsyncWebsocketClient::new(uri, &mut buffer);
    // connect
    ws.open().await.unwrap();
    // send request
    let account_info = AccountInfo::new(
        "rJumr5e1HwiuV543H7bqixhtFreChWTaHH",
        None,
        None,
        None,
        None,
        None,
        None,
    );
    ws.write(account_info).await.unwrap();

    while let Ok(Some(ReadResult::Text(response))) = ws.read().await {
        println!("{:?}", response);

        break;
    }
}
