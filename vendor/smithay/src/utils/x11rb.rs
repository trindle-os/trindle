//! Helper utilities for using x11rb as an event source in calloop.

use std::{
    sync::Arc,
    thread::{spawn, JoinHandle},
};

use tracing::{error, warn};
use x11rb::{
    connection::Connection as _,
    protocol::{
        xproto::{Atom, ClientMessageEvent, ConnectionExt as _, EventMask, Window, CLIENT_MESSAGE_EVENT},
        Event,
    },
    rust_connection::RustConnection,
};

use calloop::{
    channel::{sync_channel, Channel, ChannelError, Event as ChannelEvent, SyncSender},
    EventSource, Poll, PostAction, Readiness, Token, TokenFactory,
};

#[derive(Debug)]
pub struct X11Source {
    connection: Arc<RustConnection>,
    channel: Option<Channel<Event>>,
    event_thread: Option<JoinHandle<()>>,
    close_window: Window,
    close_type: Atom,
}

impl X11Source {
    pub fn new(connection: Arc<RustConnection>, close_window: Window, close_type: Atom) -> Self {
        let (sender, channel) = sync_channel(5);
        let conn = Arc::clone(&connection);
        let event_thread = Some(spawn(move || {
            run_event_thread(conn, sender);
        }));

        Self {
            connection,
            channel: Some(channel),
            event_thread,
            close_window,
            close_type,
        }
    }
}

impl Drop for X11Source {
    fn drop(&mut self) {
        self.channel.take();

        let event = ClientMessageEvent {
            response_type: CLIENT_MESSAGE_EVENT,
            format: 8,
            sequence: 0,
            window: self.close_window,
            type_: self.close_type,
            data: [0; 20].into(),
        };

        let _ = self
            .connection
            .send_event(false, self.close_window, EventMask::NO_EVENT, event);
        let _ = self.connection.flush();

        self.event_thread.take().map(|handle| handle.join());
    }
}

impl EventSource for X11Source {
    type Event = ChannelEvent<Event>;
    type Metadata = ();
    type Ret = ();
    type Error = ChannelError;

    #[profiling::function]
    fn process_events<C>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: C,
    ) -> Result<PostAction, ChannelError>
    where
        C: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        if let Some(channel) = &mut self.channel {
            channel.process_events(readiness, token, move |event, meta| {
                if matches!(event, ChannelEvent::Closed) {
                    warn!("Event thread exited");
                }
                callback(event, meta)
            })
        } else {
            Ok(PostAction::Remove)
        }
    }

    fn register(&mut self, poll: &mut Poll, factory: &mut TokenFactory) -> calloop::Result<()> {
        if let Some(channel) = &mut self.channel {
            channel.register(poll, factory)?;
        }

        Ok(())
    }

    fn reregister(&mut self, poll: &mut Poll, factory: &mut TokenFactory) -> calloop::Result<()> {
        if let Some(channel) = &mut self.channel {
            channel.reregister(poll, factory)?;
        }

        Ok(())
    }

    fn unregister(&mut self, poll: &mut Poll) -> calloop::Result<()> {
        if let Some(channel) = &mut self.channel {
            channel.unregister(poll)?;
        }

        Ok(())
    }
}

fn run_event_thread(connection: Arc<RustConnection>, sender: SyncSender<Event>) {
    loop {
        let event = match connection.wait_for_event() {
            Ok(event) => event,
            Err(err) => {
                error!("Event thread exiting due to connection error {}", err);
                break;
            }
        };
        match sender.send(event) {
            Ok(()) => {}
            Err(_) => {
                break;
            }
        }
    }
}
