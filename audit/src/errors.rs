use netlink_proto::NetlinkProtoError;
use packet::NetlinkMessage;
use std::io;

#[derive(Fail, Debug)]
pub enum NetlinkAuditError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "Received an unexpected message")]
    UnexpectedMessage(NetlinkMessage),

    #[fail(display = "{}", _0)]
    Protocol(NetlinkProtoError),
}

impl From<NetlinkProtoError> for NetlinkAuditError {
    fn from(err: NetlinkProtoError) -> Self {
        NetlinkAuditError::Protocol(err)
    }
}

impl From<io::Error> for NetlinkAuditError {
    fn from(err: io::Error) -> Self {
        NetlinkAuditError::Io(err)
    }
}
