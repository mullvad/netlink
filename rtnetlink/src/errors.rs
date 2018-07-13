use netlink_proto::NetlinkProtoError;
use packet::NetlinkMessage;
use std::io;

#[derive(Fail, Debug)]
pub enum NetlinkIpError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "Received an unexpected message")]
    UnexpectedMessage(NetlinkMessage),

    #[fail(
        display = "Received a link message (RTM_GETLINK, RTM_NEWLINK, RTM_SETLINK or RTMGETLINK) with an invalid hardware address attribute."
    )]
    InvalidLinkAddress(Vec<u8>),

    #[fail(display = "{}", _0)]
    Protocol(NetlinkProtoError),
}

impl From<NetlinkProtoError> for NetlinkIpError {
    fn from(err: NetlinkProtoError) -> Self {
        NetlinkIpError::Protocol(err)
    }
}
