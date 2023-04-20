mod base;
mod order;
use base::protocols::{InterOps, KanalOps, MessageType};

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

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {
        use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
        tracing_subscriber::fmt()
            // RUST_LOG=tokio=trace
            .with_env_filter(EnvFilter::from_default_env().add_directive("chat=info".parse()?))
            .with_span_events(FmtSpan::FULL)
            .init();

        // Init things

        // Surreal

        // Primary UI

        // SPSC runtime for DB and PUI

        loop {
            tokio::spawn(async move {
                tracing::debug!("Started");

                if let Err(e) = unimplemented!() {
                    tracing::info!("an error occurred; error = {:?}", e);
                }
            });
        }
    }

    async fn process() -> Result<(), Box<dyn Error>> {
        // let username = match lines.next().await {
        //     Some(Ok(line)) => line,
        //     _ => {
        //         tracing::error!("Failed to get username from {}. Client disconnected.", addr);
        //         return Ok(());
        //     }
        // };

        loop {
            tokio::select! {
                Some(msg) = peer.rx.recv() => {
                    peer.lines.send(&msg).await?;
                }
                result = peer.lines.next() => match result {
                    Some(Ok(msg)) => {

                    }
                    Some(Err(e)) => {
                        tracing::error!("error for {}; = {:?}", username, e);
                    }
                    None => break,
                },
            }
        }

        Ok(())
    }
}
