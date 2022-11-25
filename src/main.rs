mod model;
mod mp3_stream_decoder;
mod play;

use anyhow::Result;
use futures_util::StreamExt;
use model::CodeRadioMessage;
use play::Play;

const WEBSOCKET_API_URL: &str =
    "wss://coderadio-admin.freecodecamp.org/api/live/nowplaying/coderadio";

#[tokio::main]
async fn main() -> Result<()> {
    start().await
}

async fn start() -> Result<()> {
    let play = Play::try_new()?;

    let mut listen_url = Option::None;
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(WEBSOCKET_API_URL)
        .await
        .expect("Failed to connect");

    while let Some(msg) = parse_websocket_message(ws_stream.next().await).await? {
        if listen_url.is_none() {
            if !msg.station.listen_url.is_empty() {
                play.play(&msg.station.listen_url)
            }
            listen_url = Some(msg.station.listen_url);
        }
    }

    Ok(())
}

async fn parse_websocket_message(
    message: Option<
        Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>,
    >,
) -> Result<Option<CodeRadioMessage>> {
    if let Some(message) = message {
        let message: CodeRadioMessage = serde_json::from_str(message?.to_text()?)?;
        Ok(Some(message))
    } else {
        Ok(None)
    }
}
