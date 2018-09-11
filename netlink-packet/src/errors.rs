use core::fmt::{self, Display};
use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct EncodeError {
    inner: Context<EncodeErrorKind>,
}

impl Fail for EncodeError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<EncodeErrorKind> for EncodeError {
    fn from(kind: EncodeErrorKind) -> EncodeError {
        EncodeError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<EncodeErrorKind>> for EncodeError {
    fn from(inner: Context<EncodeErrorKind>) -> EncodeError {
        EncodeError { inner }
    }
}

#[derive(Fail, Debug)]
pub enum EncodeErrorKind {
    #[fail(display = "buffer is too small")]
    Exhausted,
}

#[derive(Debug)]
pub struct DecodeError {
    inner: Context<String>,
}

impl Fail for DecodeError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<String> for DecodeError {
    fn from(msg: String) -> DecodeError {
        DecodeError {
            inner: Context::new(msg),
        }
    }
}

impl<'a> From<&'a str> for DecodeError {
    fn from(msg: &'a str) -> DecodeError {
        DecodeError {
            inner: Context::new(msg.into()),
        }
    }
}

impl<'a> From<Context<&'a str>> for DecodeError {
    fn from(inner: Context<&'a str>) -> DecodeError {
        DecodeError {
            inner: Context::new(inner.get_context().to_string()),
        }
    }
}

impl From<Context<String>> for DecodeError {
    fn from(inner: Context<String>) -> DecodeError {
        DecodeError { inner }
    }
}
