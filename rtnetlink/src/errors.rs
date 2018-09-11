use failure::{Backtrace, Context, Fail};
use packet::NetlinkMessage;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Received an unexpected message {:?}", _0)]
    UnexpectedMessage(NetlinkMessage),

    #[fail(
        display = "Received a link message (RTM_GETLINK, RTM_NEWLINK, RTM_SETLINK or RTMGETLINK) with an invalid hardware address attribute: {:?}.",
        _0
    )]
    InvalidHardwareAddress(Vec<u8>),

    #[fail(display = "Request failed")]
    RequestFailed,
}

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
