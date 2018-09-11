use std::fmt::{self, Display};

use failure::{Backtrace, Context, Fail};

use netlink_packet::NetlinkMessage;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Failed to send a netlink packet")]
    Emit,

    #[fail(display = "The netlink connection is closed")]
    ConnectionClosed,

    #[fail(
        display = "Received an error message as a response: {:?}",
        _0
    )]
    NetlinkError(NetlinkMessage),
}

// Below is the boilerplate from https://boats.gitlab.io/failure/error-errorkind.html

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.inner.get_context().clone()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}
