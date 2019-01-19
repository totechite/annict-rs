use crate::{Service, Error};
use serde::Serialize;
use std::cmp::PartialEq;

/// A client to make request with Service.
///
/// Examples
/// ========
/// ```rust
/// # use annis::{Client, Value, Error};
/// #
/// # fn run() -> Result<(), Error> {
/// let client = Client::set_token("access_token");
/// let res = client.call(annis::works())?.json()?;
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

    pub fn call<K>(&self, service: Service<K>) -> Result<reqwest::Response, Error>
    where
        K: Serialize + Into<String> + PartialEq,
    {
        let mut client = reqwest::Client::new()
            .request(service.method, service.url.as_str())
            .query(&vec![("access_token", self.clone().token)]);
        if let Some(params) = service.params {
            client = client.query(&params);
        };
        client.send().map_err(Into::into)
    }
}

