use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc;
use crate::input::{TelemetryEvent, map_telemetry_to_egui};

pub async fn run_server(
    tx_frames: mpsc::Receiver<Vec<u8>>,
    rx_telemetry: mpsc::Sender<egui::Event>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    println!("[LAGOS_PORT: {}]", addr.port());

    let tx_frames = Arc::new(tokio::sync::Mutex::new(tx_frames));

    loop {
        let (stream, _) = listener.accept().await?;
        let tx_frames = tx_frames.clone();
        let rx_telemetry = rx_telemetry.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, tx_frames, rx_telemetry).await {
                log::error!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    tx_frames: Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>,
    rx_telemetry: mpsc::Sender<egui::Event>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let mut tx_frames = tx_frames.lock().await;

    loop {
        tokio::select! {
            Some(frame) = tx_frames.recv() => {
                ws_sender.send(Message::Binary(frame)).await?;
            }
            Some(msg) = ws_receiver.next() => {
                let msg = msg?;
                if let Message::Text(text) = msg {
                    if let Ok(event) = serde_json::from_str::<TelemetryEvent>(&text) {
                        for egui_event in map_telemetry_to_egui(event) {
                            let _ = rx_telemetry.send(egui_event).await;
                        }
                    }
                }
            }
            else => break,
        }
    }

    Ok(())
}
