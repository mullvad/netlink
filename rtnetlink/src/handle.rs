use futures::{Future, Stream};
use netlink_proto::ConnectionHandle;
use packet::NetlinkMessage;

use {LinkHandle, NetlinkIpError};

#[derive(Clone, Debug)]
pub struct Handle(ConnectionHandle);

impl Handle {
    pub(crate) fn new(conn: ConnectionHandle) -> Self {
        Handle(conn)
    }

    pub fn request(
        &mut self,
        message: NetlinkMessage,
    ) -> impl Stream<Item = NetlinkMessage, Error = NetlinkIpError> {
        self.0.request(message).map_err(From::from)
    }

    pub fn buffered_request(
        &mut self,
        msg: NetlinkMessage,
    ) -> impl Future<Item = Vec<NetlinkMessage>, Error = NetlinkIpError> {
        self.0.buffered_request(msg).map_err(|e| e.into())
    }

    pub fn acked_request(
        &mut self,
        msg: NetlinkMessage,
    ) -> impl Future<Item = (), Error = NetlinkIpError> {
        self.0.acked_request(msg).map_err(|e| e.into())
    }

    /// Create a new handle, specifically for link requests (equivalent to `ip link` commands)
    pub fn link(&self) -> LinkHandle {
        LinkHandle::new(self.clone())
    }
}
