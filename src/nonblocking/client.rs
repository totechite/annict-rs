use crate::{Error, Service};
use futures::TryFutureExt;
use reqwest::Client as AsyncClient;
use serde::Serialize;
use std::cmp::PartialEq;

/// A client to make asynchronous request with Service.
///
/// Examples
/// ========
/// ```rust
/// # use annis::{Value, Error};
/// # use annis::nonblocking::Client;
/// # use tokio;
/// #
/// # #[tokio::main]
/// # async fn run() -> Result<(), Error> {
/// let client = Client::set_token("access_token");
/// let res = client.call(annis::works()).await?.json().await?;
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

    pub async fn call<K>(&self, service: Service<K>) -> Result<reqwest::Response, Error>
    where
        K: Serialize + Into<String> + PartialEq,
    {
        let mut client = AsyncClient::new()
            .request(service.method, service.url.as_str())
            .query(&vec![("access_token", self.clone().token)]);
        if let Some(params) = service.params {
            client = client.query(&params);
        };
        client.send().map_err(Into::into).await
    }
}
