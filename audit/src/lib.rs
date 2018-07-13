#![cfg_attr(feature = "cargo-clippy", allow(module_inception))]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate eui48;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate futures;
extern crate tokio_core;

pub extern crate netlink_packet as packet;
pub use packet::constants;
extern crate netlink_proto;
pub use netlink_proto::Connection;

mod handle;
pub use handle::*;

mod errors;
pub use errors::*;

pub fn new_connection() -> Result<(Connection, Handle), NetlinkAuditError> {
    let (conn, handle) = netlink_proto::new_connection()?;
    Ok((conn, Handle::new(handle)))
}
