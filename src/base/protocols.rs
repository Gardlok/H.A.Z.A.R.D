#![warn(rust_2018_idioms)]
#![feature(async_fn_in_trait)]
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    env,
    error::Error as StdError,
    hash::{Hash, Hasher},
    io,
    net::SocketAddr,
};

use anyhow::{Error, Result};

use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
//use bincode;  //TODO

use async_tungstenite::accept_async;
use futures::SinkExt;
use kanal::{AsyncReceiver, AsyncSender, ReceiveError, SendError};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::{Error as TungError, Message, Result as TungResult};
//use axum;     //TODO

// ////////////////////////////////////////////////////////////////////////
// Common
// /////////////////////////////////////////////////////////
pub enum MessageType {
    Poll,
    Push,
    Pull,
}

// ////////////////////////////////////////////////////////////////////////
// LoKal Interopts
// /////////////////////////////////////////////////////////

struct Id(usize);

pub struct KanalComm {
    id: Id,
    broadkast: KanalOps,
    directs: HashMap<usize, AsyncSender<MessageType>>,
    subsciptions: HashMap<usize, AsyncReceiver<MessageType>>,
}

pub struct KanalOps {
    id: usize,
    tx: AsyncSender<MessageType>,
    rx: AsyncReceiver<MessageType>,
}

pub trait InterOps {
    fn new(size: Option<usize>) -> Self;
    fn new_paired(size: Option<usize>) -> (Self, Self);
    fn share() -> Self;
    fn register() -> Result<(), ReceiveError>;
}

impl InterOps for KanalOps {
    fn new(capacity: Option<usize>) -> KanalOps {
        let (tx, rx) = match capacity {
            None => kanal::unbounded_async(),
            Some(cap) => kanal::bounded_async(cap),
        };
        KanalOps { id: 0, tx, rx }
    }

    fn new_paired(capacity: Option<usize>) -> (KanalOps, KanalOps) {
        let (tx_a, rx_a, tx_b, rx_b) = match capacity {
            None => {
                let (tx_a, rx_a) = kanal::unbounded_async();
                let (tx_b, rx_b) = kanal::unbounded_async();
                (tx_a, rx_b, tx_b, rx_a)
            }
            Some(cap) => {
                let (tx_a, rx_a) = kanal::unbounded_async();
                let (tx_b, rx_b) = kanal::unbounded_async();
                (tx_a, rx_b, tx_b, rx_a)
            }
        };

        KanalOps {
            id: 0,
            tx: tx_a,
            rx: rx_a,
        };
        KanalOps {
            id: 0,
            tx: tx_b,
            rx: rx_b,
        };
    }

    fn share(original: KanalOps) -> KanalOps {
        original.clone()
    }

    async fn register(self) -> Result<(), ReceiveError> {
        self.tx.send(MessageType::Poll).await?;
        self.id = self.rx.recv().await?
    }
}

// ////////////////////////////////////////////////////////////////////////
// Websockets
// /////////////////////////////////////////////////////////
/*
async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    mut event_tx: UnboundedSender<GameEvent>,
) -> TungResult<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = unbounded();

    ws_sender
        .send(Message::Text(
            serde_json::to_string(&GameData::ReqLogin).unwrap(),
        ))
        .await?;
    if let Ok(GameEvent::Auth(token)) =
        serde_json::from_str(ws_receiver.next().await.unwrap()?.to_text()?)
    {
        if let Some(p) = try_current(&token.to_string()) {
            println!("Logging in {} to WS stream", p.user.username);
            event_tx
                .send(GameEvent::Login(Some(p.user.id), tx.clone()))
                .await
                .unwrap();
        } else {
            println!("Logging in viewer to WS stream");
            event_tx
                .send(GameEvent::Login(None, tx.clone()))
                .await
                .unwrap();
        }
    } else {
        println!("Logging in viewer to WS stream");
        event_tx
            .send(GameEvent::Login(None, tx.clone()))
            .await
            .unwrap();
    }

    let mut ws_msg_fut = ws_receiver.next();
    let mut game_msg_fut = rx.next();

    loop {
        match select(ws_msg_fut, game_msg_fut).await {
            // GameEvents coming in from the Client to the game loop
            Either::Left((ws_msg, game_msg_fut_continue)) => {
                match ws_msg {
                    Some(ws_msg) => {
                        let ws_msg = ws_msg?;
                        if ws_msg.is_text() || ws_msg.is_binary() {
                            //println!("{:?}", ws_msg.to_text());
                            if let Ok(ws_msg_) = serde_json::from_str(ws_msg.to_text().unwrap()) {
                                event_tx.send(ws_msg_).await.unwrap();
                            }
                        } else if ws_msg.is_close() {
                            break;
                        }
                        game_msg_fut = game_msg_fut_continue; // Continue waiting for tick.
                        ws_msg_fut = ws_receiver.next(); // Receive next WebSocket message.
                    }
                    None => break, // WebSocket stream terminated.
                };
            }

            // GameData coming from the game loop to the client
            Either::Right((game_msg, ws_msg_fut_continue)) => {
                ws_sender
                    .send(Message::Text(
                        serde_json::to_string(&game_msg.unwrap()).unwrap(),
                    ))
                    .await?;
                ws_msg_fut = ws_msg_fut_continue; // Continue receiving the WebSocket message.
                game_msg_fut = rx.next(); // Wait for next tick.
            }
        }
    }
    Ok(())
}

async fn accept_connection(
    peer: SocketAddr,
    stream: TcpStream,
    event_tx: UnboundedSender<GameEvent>,
) {
    if let Err(e) = handle_connection(peer, stream, event_tx).await {
        match e {
            TungError::ConnectionClosed | TungError::Protocol(_) | TungError::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

/// Listens for incoming connections and serves them.
async fn ws_listen(event_tx: UnboundedSender<GameEvent>, listener: TcpListener) -> Result<()> {
    let host = listener.local_addr();
    println!("Listening on {:?}", host);
    loop {
        // Accept the next connection.
        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            println!("Peer address: {}", peer);
            smol::spawn(accept_connection(peer, stream, event_tx.clone())).detach();
        }
    }
}
*/

// ////////////////////////////////////////////////////////////////////////
// Indexing
// /////////////////////////////////////////////////////////
