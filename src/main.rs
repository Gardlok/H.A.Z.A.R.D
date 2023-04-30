#![feature(allocator_api)]
#![feature(async_fn_in_trait)]
mod base;
mod order;
use base::logging::Logging;
use base::protocols::{InterOps, MessageType};

use order::pallet::home;

use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use kanal::{AsyncReceiver, AsyncSender, ReceiveError, SendError};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;

use console_engine::{
    events::Event,
    forms::{Checkbox, Form, FormField, FormOptions, FormStyle, FormValue, Radio},
    rect_style::BorderStyle,
    ConsoleEngine, KeyCode, KeyModifiers,
};
use crossterm::event::KeyEvent;

/*
// DB
pub struct SurrealDeal {
    database: surrealdb::engine::local::Db,
    sharedkast: SharedKast<MessageType>,
    directkast: HashMap<u8, DirectKast<MessageType>>,
}

impl SurrealDeal {
    pub async fn new() -> SurrealDeal {
        // Create channels
        let dx = SharedKast::new();

        SurrealDeal {
            database: Surreal::new::<Mem>(()).await?,
            sharedkast: SharedKast::new(),
            directkast: HashMap::new(),
        }
    }

    */
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Logging::setup();
    tracing::info!("test trace!");
    Ok(())
}

async fn process() -> Result<(), Box<dyn Error>> {
    unimplemented!();
    loop { /*
         tokio::select! {
             Some(msg) = peer.rx.recv() => {
                 //peer.lines.send(&msg).await?;
             //result = peer.lines.next() => match result {
                 Some(Ok(msg)) => {

                 }
                 Some(Err(e)) => {
                     tracing::error!("error for {}; = {:?}", "bob", e);
                 }
                 None => break,
             },
         }*/
    }
    Ok(())
}
