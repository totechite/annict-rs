use crate::Service;
use crate::Value;
use serde::Serialize;
use std::cmp::PartialEq;

/// A client to make request with Service.
///
/// Examples
/// ========
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// let client = Client::set_token("access_token");
/// let res = client.call(annis::works())?;
/// #   Ok(())
/// # }
///```

#[derive(Debug, Clone)]
pub struct Client {
    pub token: String,
}

impl Client {
    pub fn set_token<T>(access_token: T) -> Self
    where
        T: Into<String>,
    {
        Client {
            token: access_token.into(),
        }
    }

    pub fn call<K>(&self, service: Service<K>) -> Result<Value, Error>
    where
        K: Serialize + Into<String> + PartialEq,
    {
        let mut client = reqwest::Client::new()
            .request(service.method, service.url.as_str())
            .query(&vec![("access_token", self.clone().token)]);
        if let Some(params) = service.params {
            client = client.query(&params);
        };
        let mut res = client.send()?;
        res.json::<crate::Value>().map_err(Into::into)
    }
}

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "error: Invalid value at token or request parameters")]
    InvalidValue,
}

/* ----------- failure boilerplate ----------- */

use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
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
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
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

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error {
            inner: error.context(ErrorKind::InvalidValue),
        }
    }
}
