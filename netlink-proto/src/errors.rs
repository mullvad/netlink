use core;
use netlink_packet::{NetlinkMessage, NetlinkPacketError};
use std::io;

#[derive(Fail, Debug)]
pub enum NetlinkProtoError {
    #[fail(display = "Failed to send a netlink packet: {}", _0)]
    Emit(#[cause] NetlinkPacketError),

    #[fail(display = "The netlink connection is closed")]
    ConnectionClosed,

    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(
        display = "Received an error message as a response: {:?}",
        _0
    )]
    ErrorMessage(NetlinkMessage),
}

impl From<io::Error> for NetlinkProtoError {
    fn from(io_err: io::Error) -> NetlinkProtoError {
        NetlinkProtoError::Io(io_err)
    }
}

pub type Result<T> = core::result::Result<T, NetlinkProtoError>;
