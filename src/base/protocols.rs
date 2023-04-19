#![warn(rust_2018_idioms)]
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    net::SocketAddr,
    std::env,
    std::error::Error,
    std::io,
};

use anyhow::{Error, Result};

use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
//use bincode;  //TODO

use async_tungstenite::accept_async;
use futures::SinkExt;
use kanal::{bounded_async, unbounded_async, AsyncReceiver, AsyncSender, ReceiveError, SendError};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::{Error as TungError, Message, Result as TungResult};
//use axum;     //TODO

// ////////////////////////////////////////////////////////////////////////
// Interopts
// /////////////////////////////////////////////////////////
pub enum MessageType<T> {
    Poll,
    Push,
    Pull,
}

pub trait Caster<T> {
    fn new() -> Option<T>;
    fn share(T: Self) -> Option<T>;
}

pub struct InteropComm {
    shared: SharedCast<MessageType>,
    directs: HashMap<id: u8, DirectCast<T>>,
    subsciptions: HashMap<id: u8, AsyncReceiver<MessageType>>,
}

pub type SharedCast<T: MessageType> = (AsyncSender<T>, AsyncReceiver<T>);
pub type DirectCast<T> = (AsyncSender<T>, AsyncReceiver<T>);
pub type SubCast<T: MessageType> = (AsyncSender<T>, AsyncReceiver<T>);

impl Caster for SharedCast {
    fn new() -> SharedCast {
        let (tx, rx) = kanal::unbounded_async::<MessageType>();
        SharedCast { tx, rx }
    }

    fn share(original: Caster<T>) -> Option<Caster<T>>
    where
        T: Clone,
    {
        let replica = original.clone();
        Some(replica)
    }
}

impl Caster for DirectCast<T> {
    fn new(size: usize) -> (DirectCast<T>, DirectCast<T>) {
        let (tx_a, rx_a) = kanal::bounded_async::<T>(size);
        let (tx_b, rx_b) = kanal::bounded_async::<T>(size);
        (
            DirectCast { tx: tx_a, rx: rx_b },
            DirectCast { tx: tx_b, rx: rx_a },
        )
    }

    fn share(original: Caster<T>) -> Option<Caster<T>>
    where
        T: Clone,
    {
        let replica = original.clone();
        Some(replica)
    }
}

impl caster for SubCast<T> {
    fn new() -> (SubCast<T>, SubCast<T>) {
        let (service_tx, subscribe_rx) = kanal::unbounded_async();

        (
            SubCast {
                service_tx,
                subscribe_rx,
            },
            SubCast { tx: tx_b, rx: rx_a },
        )
    }

    fn share(original: Caster<T>) -> Option<Caster<T>>
    where
        T: Clone,
    {
        let replica = original.clone();
        Some(replica)
    }
}

// ////////////////////////////////////////////////////////////////////////
// Websockets
// /////////////////////////////////////////////////////////

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

// ////////////////////////////////////////////////////////////////////////
// Indexing
// /////////////////////////////////////////////////////////
