//! An optional high level API for working with LFS through Insim.
//!
//! :warning: API is not stable.
//!
//! [Client] is a wrapper around [Transport](super::protocol::transport::Transport) which is able to
//! transparently reconnect to the Insim server, with a configurable backoff.
//!
//! Rather than a stream of [Packet](super::protocol::Packet) it instead provides a stream of [Event] which
//! describes the current state of the client.
//!
//! You create and configure [Client] through [Config].
//!
//! # Example
//! ```rust
//! use futures::{SinkExt, StreamExt};
//! use insim;
//! use tracing_subscriber;
//!
//! #[tokio::main]
//! pub async fn main() {
//!
//!     // Make a connection to the Insim Relay
//!     let client = insim::client::Config::default()
//!         .relay()
//!         .build();
//!
//!     // We MUST poll the future to ensure that the client stays connected
//!     // Once the client is shutdown it will output an Event::Shutdown and then return None.
//!     while let Some(m) = client.next().await {
//!         match m {
//!             insim::client::Event::Connected => {
//!                 let _ = client
//!                     .send(insim::client::Event::Packet(
//!                         insim::protocol::relay::HostSelect {
//!                             hname: "Nubbins AU Demo".into(),
//!                             ..Default::default()
//!                         }
//!                         .into(),
//!                     ))
//!                     .await;
//!             }
//!             _ => {
//!               tracing::debug!("Event: {:?}", m);
//!             }
//!         }
//!     }
//! }
//! ```

pub(crate) mod actor;
pub(crate) mod config;

pub use config::Config;

use crate::{error, protocol};
use flume;
use std::cell::Cell;
use tokio::task::JoinHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Startup,
    Connecting,
    Handshaking,
    Connected,
    Disconnected,
    Shutdown,
}

#[derive(Debug)]
pub enum Event {
    State(State),
    Frame(protocol::Packet),
    Error(error::Error),
}

pub struct Client {
    receiver: flume::Receiver<Event>,
    sender: flume::Sender<Event>,
    handle: JoinHandle<()>,
    state: Cell<State>,
}

impl Client {
    pub fn from_config(config: Config) -> Self {
        let (actor_tx, actor_rx) = flume::unbounded();
        let (client_tx, client_rx) = flume::unbounded();

        let mut actor = actor::ClientActor {
            config,
            receiver: client_rx,
            sender: actor_tx,
            attempt: 0,
        };

        let handle = tokio::spawn(async move {
            actor.run().await;
        });

        Self {
            receiver: actor_rx,
            sender: client_tx,
            handle,
            state: Cell::new(State::Startup),
        }
    }

    pub async fn send(&self, e: Event) {
        self.sender.send_async(e).await.expect("failed to send");
    }

    pub async fn next(&self) -> Option<Event> {
        return self.recv().await;
    }

    pub async fn recv(&self) -> Option<Event> {
        match self.receiver.recv_async().await {
            Ok(Event::State(e)) => {
                self.state.replace(e);
                Some(Event::State(e))
            }
            Ok(e) => Some(e),
            Err(e) => panic!("unhandled error during recv {}", e),
        }
    }

    pub async fn shutdown(self) {
        self.send(Event::State(State::Shutdown)).await;
        self.handle.await.expect("failed to join actor handle");
    }

    pub fn is_connected(&self) -> bool {
        self.state.get() == State::Connected
    }

    pub fn is_shutdown(&self) -> bool {
        self.state.get() == State::Shutdown
    }
}
