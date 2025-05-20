use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Read user input and send to server
            line = stdin.next_line() => {
                let msg = line?.unwrap_or_default();
                if msg.is_empty() {
                    continue;
                }
                ws_stream.send(Message::text(msg)).await?;
            }

            // Receive messages from server
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("Zakiy's computer from server: {text}");
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("WebSocket error: {e}");
                        break;
                    }
                    None => {
                        println!("Connection closed by server.");
                        break;
                    }
                }
            }

        }
    }

    // Never reached, but required for return type
    // (If the loop breaks, you can return Ok(()) above)
    #[allow(unreachable_code)]
    Ok(())
}
