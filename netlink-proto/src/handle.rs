use std::mem;

use futures::sync::mpsc::{unbounded, UnboundedSender};
use futures::{Async, Future, Poll, Stream};
use netlink_packet::NetlinkMessage;

use errors::{Error, ErrorKind};

type RequestsTx = UnboundedSender<(UnboundedSender<NetlinkMessage>, NetlinkMessage)>;

/// A handle to pass requests to a [`Connection`](struct.Connection.html).
#[derive(Clone, Debug)]
pub struct ConnectionHandle {
    requests_tx: RequestsTx,
}

impl ConnectionHandle {
    pub fn new(requests_tx: RequestsTx) -> Self {
        ConnectionHandle { requests_tx }
    }

    /// Send a new request and get the response as a stream of messages. Note that some messages
    /// are not part of the response stream:
    ///
    /// - **acknowledgements**: when an acknowledgement is received, the stream is closed
    /// - **end of dump messages**: similarly, upon receiving an "end of dump" message, the stream is
    /// closed
    pub fn request(
        &mut self,
        msg: NetlinkMessage,
    ) -> impl Stream<Item = NetlinkMessage, Error = Error> {
        let (tx, rx) = unbounded::<NetlinkMessage>();
        // Ignore the result. If this failed, `tx` will be dropped when this funtion returns, and
        // polling rx with fail, carrying the error.
        debug!("handle: forwarding new request to connection");
        let _ = UnboundedSender::unbounded_send(&self.requests_tx, (tx, msg));
        rx.map_err(|()| {
            error!("could not forward new request to connection: the connection is closed");
            ErrorKind::ConnectionClosed.into()
        })
    }

    pub fn buffered_request(
        &mut self,
        msg: NetlinkMessage,
    ) -> impl Future<Item = Vec<NetlinkMessage>, Error = Error> {
        Stream2Vec::new(self.request(msg))
    }

    pub fn acked_request(&mut self, msg: NetlinkMessage) -> impl Future<Item = (), Error = Error> {
        self.request(msg).for_each(|msg| {
            if msg.is_error() {
                Err(ErrorKind::NetlinkError(msg).into())
            } else {
                Ok(())
            }
        })
    }
}

/// A future that polls a `Stream` until the end, and return all the items in a `Vec`
pub struct Stream2Vec<S, T>(S, Option<Vec<T>>);

impl<S, T> Stream2Vec<S, T> {
    pub(crate) fn new(s: S) -> Self {
        Stream2Vec(s, Some(vec![]))
    }
}

impl<S: Stream<Item = T, Error = Error>, T> Future for Stream2Vec<S, T> {
    type Item = Vec<T>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.0.poll()? {
                Async::Ready(Some(item)) => self.1.as_mut().unwrap().push(item),
                Async::Ready(None) => {
                    return Ok(Async::Ready(mem::replace(&mut self.1, None).unwrap()))
                }
                Async::NotReady => return Ok(Async::NotReady),
            }
        }
    }
}

// pub struct Stream2Ack<S>(S);
//
// impl<S> Stream2Ack<S> {
//     pub(crate) fn new(s: S) -> Self {
//         Stream2Ack(s)
//     }
// }
//
// impl<S> Future for Stream2Ack<S>
// where
//     S: Stream<Item = NetlinkMessage, Error = Error>,
// {
//     type Item = ();
//     type Error = Error;
//
//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         match self.0.poll()? {
//             Async::NotReady => Ok(Async::NotReady),
//             // If the stream closes right away, that means we received an ack
//             Async::Ready(None) => Ok(Async::Ready(())),
//             Async::Ready(Some(msg)) => {
//                 if msg.is_error() {
//                     Err(ErrorKind::NetlinkError(msg.clone()))
//                 } else {
//                     Err(ErrorKind::NetlinkError(msg))
//                 }
//             }
//         }
//     }
// }
