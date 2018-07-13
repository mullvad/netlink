use futures::Stream;

use packet::constants::{NLM_F_DUMP, NLM_F_REQUEST};
use packet::{LinkMessage, NetlinkFlags, NetlinkMessage, NetlinkPayload, RtnlMessage};

use super::Link;
use {Handle, NetlinkIpError};

lazy_static! {
    // Flags for `ip link get`
    static ref GET_FLAGS: NetlinkFlags = NetlinkFlags::from(NLM_F_REQUEST | NLM_F_DUMP);
}

pub struct LinkGetRequest {
    handle: Handle,
    message: LinkMessage,
}

impl LinkGetRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        let message = LinkMessage::new();
        LinkGetRequest { handle, message }
    }

    /// Execute the request
    pub fn execute(self) -> impl Stream<Item = Link, Error = NetlinkIpError> {
        let LinkGetRequest {
            mut handle,
            message,
        } = self;
        let mut req = NetlinkMessage::from(RtnlMessage::GetLink(message));
        req.header_mut().set_flags(*GET_FLAGS);
        handle.request(req).and_then(move |msg| {
            let (header, payload) = msg.into_parts();
            if let NetlinkPayload::Rtnl(RtnlMessage::NewLink(msg)) = payload {
                Ok(Link::from_link_message(msg)?)
            } else {
                Err(NetlinkIpError::UnexpectedMessage(NetlinkMessage::new(
                    header, payload,
                )))
            }
        })
    }

    /// Return a mutable reference to the request
    pub fn message_mut(&mut self) -> &mut LinkMessage {
        &mut self.message
    }
}
