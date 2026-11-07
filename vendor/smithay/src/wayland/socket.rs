//! Wayland listening socket.

use std::{ffi::OsStr, io, os::unix::net::UnixStream};

use calloop::{
    generic::Generic, EventSource, Interest, Mode, Poll, PostAction, Readiness, Token, TokenFactory,
};
use tracing::{debug, info};
use wayland_server::{BindError, ListeningSocket};

#[derive(Debug)]
pub struct ListeningSocketSource {
    socket: Generic<ListeningSocket>,
}

impl ListeningSocketSource {
    pub fn new_auto() -> Result<ListeningSocketSource, BindError> {
        let socket = ListeningSocket::bind_auto("wayland", 1..33)?;

        info!(name = ?socket.socket_name(), "Created new socket");

        Ok(ListeningSocketSource {
            socket: Generic::new(socket, Interest::READ, Mode::Level),
        })
    }

    pub fn with_name(name: &str) -> Result<ListeningSocketSource, BindError> {
        let socket = ListeningSocket::bind(name)?;
        info!(name = ?socket.socket_name(), "Created new socket");

        Ok(ListeningSocketSource {
            socket: Generic::new(socket, Interest::READ, Mode::Level),
        })
    }

    pub fn socket_name(&self) -> &OsStr {
        self.socket.get_ref().socket_name().unwrap()
    }
}

impl EventSource for ListeningSocketSource {
    type Event = UnixStream;
    type Metadata = ();
    type Ret = ();
    type Error = io::Error;

    fn process_events<F>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: F,
    ) -> io::Result<PostAction>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        self.socket.process_events(readiness, token, |_, socket| {
            while let Some(client) = socket.accept()? {
                debug!(socket = ?socket.socket_name(), client = ?client, "New client connected");
                callback(client, &mut ());
            }

            Ok(PostAction::Continue)
        })
    }

    fn register(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> calloop::Result<()> {
        self.socket.register(poll, token_factory)
    }

    fn reregister(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> calloop::Result<()> {
        self.socket.reregister(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut Poll) -> calloop::Result<()> {
        self.socket.unregister(poll)
    }
}
