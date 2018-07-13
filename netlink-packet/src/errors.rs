use core;

#[derive(Fail, Debug)]
pub enum NetlinkPacketError {
    #[fail(display = "A netlink packet could not be parsed")]
    Decode,

    #[fail(display = "A netlink packet could not be encoded")]
    Encode,
}

pub type Result<T> = core::result::Result<T, NetlinkPacketError>;
