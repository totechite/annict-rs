use serde::Serialize;
use Service;
use Value;
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

    pub fn call<K: Into<String> + PartialEq>(&self, service: Service<K>) -> Result<Value, String>
    where
        K: Serialize,
    {
        let mut client = service.client.bearer_auth(self.clone().token);
        if let Some(params) = service.params {
            client = client.query(&params);
        };
        let mut req = try!(client.send().map_err(|err| err.to_string()).or(Err(
            "Invalid values at token or request parameters".to_string()
        )));
        req.json::<Value>().or(Ok(Value::Null))
    }
}
