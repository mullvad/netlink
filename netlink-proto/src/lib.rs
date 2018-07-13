#![cfg(any(feature = "audit", feature = "rtnetlink"))]

extern crate bytes;
extern crate core;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate tokio_io;

#[cfg(any(feature = "audit", feature = "rtnetlink"))]
extern crate netlink_packet;
#[cfg(any(feature = "audit", feature = "rtnetlink"))]
extern crate netlink_sys;

mod codecs;
pub use codecs::*;

mod framed;
pub use framed::*;

mod connection;
pub use connection::*;

mod errors;
pub use errors::*;

mod handle;
pub use handle::*;

use futures::sync::mpsc::{unbounded, UnboundedSender};
use netlink_packet::NetlinkMessage;
use std::io;

pub fn new_connection() -> io::Result<(Connection, ConnectionHandle)> {
    let (tx, rx) = unbounded::<(UnboundedSender<NetlinkMessage>, NetlinkMessage)>();
    Ok((Connection::new(rx)?, ConnectionHandle::new(tx)))
}
