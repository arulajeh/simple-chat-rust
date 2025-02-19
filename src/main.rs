use std::borrow::Cow;
use futures_util::{stream::StreamExt, SinkExt};
use http::Response;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::Request;
use tokio_tungstenite::tungstenite::Message;
use url::form_urlencoded;
use ammonia::{clean, Builder};
use urlencoding::decode;

type PeerMap = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("WebSocket server is running on ws://{}", addr);

    let (tx, _rx) = broadcast::channel(16);
    let peers: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let peers = peers.clone();
        let user_id = Arc::new(Mutex::new(String::new()));

        let user_id_clone = user_id.clone();
        tokio::spawn(async move {
            let callback = move |req: &Request, res: Response<()>| -> Result<Response<()>, Response<Option<String>>> {
                if let Some(query) = req.uri().query() {
                    let params: HashMap<_, _> = form_urlencoded::parse(query.as_bytes()).into_owned().collect();
                    if let Some(id) = params.get("user_id") {
                        let mut user_id_lock = user_id_clone.lock().unwrap();
                        *user_id_lock = id.clone();
                        println!("User-Id: {}", id);
                        return Ok(res);
                    }
                }

                let response = Response::builder()
                    .status(400)
                    .body(Some("Missing or invalid User-Id".to_string()))
                    .unwrap();
                Err(response)
            };

            match accept_hdr_async(stream, callback).await {
                Ok(ws_stream) => {
                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
                    let user_id = escape_html(&user_id.lock().unwrap().clone());
                    println!("New WebSocket connection with User-Id: {}", user_id);

                    let mut rx = tx.subscribe();
                    {
                        let mut peers_lock = peers.lock().expect("Failed to lock peers mutex");
                        peers_lock.insert(user_id.clone(), tx.clone());
                    }

                    if let Err(err) = ws_sender.send(Message::Text(format!("Welcome, {}!", user_id).into())).await {
                        println!("Error sending welcome message: {:?}", err);
                        return;
                    }

                    loop {
                        tokio::select! {
                            msg = ws_receiver.next() => {
                                if let Some(Ok(msg)) = msg {
                                    if msg.is_text() {
                                        if let Ok(text) = msg.to_text() {
                                            let text = escape_html(&text.to_string());
                                            println!("Received message from {}: {}", user_id, text);
                                            let _ = tx.send(format!("{}: {}", user_id, text));
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                            msg = rx.recv() => {
                                if let Ok(msg) = msg {
                                    if ws_sender.send(Message::Text(escape_html(&msg).into())).await.is_err() {
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    println!("WebSocket connection closed: {}", user_id);
                    peers.lock().unwrap().remove(&user_id);
                }
                Err(e) => println!("Handshake failed: {:?}", e),
            }
        });
    }
}

fn escape_html(input: &str) -> String {
    let input = decode(input).unwrap_or_else(|_| Cow::from(input));
    Builder::default().tags(HashSet::new()).clean(&*input).to_string()
}
