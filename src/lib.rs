#![doc(html_root_url = "https://!docs.rs/annis/0.0.1")]

//! annis
//! =====
//!
//! The `annis` is a Rust interface to the Annict API.
//! [Annict API Official Document](https://docs.annict.com/)
//!
//! Request to /v1/works
//! --------------------
//!
//! ```rust
//! # extern crate annis;
//! # use annis::Client;
//! # use std::env;
//! #
//! # fn main() -> Result<(), String>{
//! #
//! let client = Client::set_token("annict_access_token");
//!
//! let params = vec![("filter_title", "lain"),("fields","title")];
//! let works = annis::works().params(params);
//!
//! let json = client.call(works)?;
//!
//! println!("{:?}", json["works"]);
//! #
//! #   Ok(())  
//! # }
//! ```

extern crate reqwest;
use reqwest::RequestBuilder;
extern crate serde_json;
mod auth;
mod client;

pub use auth::*;
pub use client::Client;
pub use serde_json::Value;

/// A Service to make request to endpoint.
///
///

#[derive(Debug)]
pub struct Service {
    pub client: RequestBuilder,
    pub params: Option<Vec<(String, String)>>,
}

impl Service {
    pub fn params<K, V>(self, params: Vec<(K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let mut params: Vec<(String, String)> = params
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        if let Some(mut x) = self.params {
            params.append(&mut x);
        };
        Self {
            params: Some(params),
            ..self
        }
    }
}

/// A type of argument for me_records().

pub enum Method {
    Post,
    Patch,
    Delete,
}

/// Request to /v1/works
///
/// Examples
/// ========
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let works = annis::works().params(vec![("filter_title", "lain")]);
///
/// client.call(works)?;
/// # Ok(())
/// # }
/// ```

pub fn works() -> Service {
    Service {
        client: reqwest::Client::new().get("https://api.annict.com/v1/works"),
        params: None,
    }
}

/// Request to /v1/episodes
///
/// Examples
/// ========
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let episodes = annis::episodes().params(vec![("filter_work_id", "2274")]);
///
/// client.call(episodes)?;
/// # Ok(())
/// # }
/// ```

pub fn episodes() -> Service {
    Service {
        client: reqwest::Client::new().get("https://api.annict.com/v1/episodes"),
        params: None,
    }
}

/// Request to /v1/records
///
/// Examples
/// ========
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let records = annis::records().params(vec![("fields", "title")]);
///
/// client.call(records)?;
/// # Ok(())
/// # }
/// ```

pub fn records() -> Service {
    Service {
        client: reqwest::Client::new().get("https://api.annict.com/v1/records"),
        params: None,
    }
}

/// Request to /v1/me/statuses
///
/// Examples
/// ========
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let statuses = annis::me_statuses().params(vec![("work_id", "3994"), ("kind", "watched")]);
///
/// client.call(statuses)?;
/// # Ok(())
/// # }
/// ```

pub fn me_statuses() -> Service {
    Service {
        client: reqwest::Client::new().post("https://api.annict.com/v1/me/statuses"),
        params: None,
    }
}

/// Request to /v1/me/records
///
/// Examples
/// ========
/// POST
/// ```rust
/// # use annis::{Client, Method};
/// #
/// # fn post() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let records = annis::me_records(Method::Post, 5013).params(vec![("episode_id", "5013"), ("rating", "5")]);
///
/// client.call(records)?;
/// # Ok(())
/// # }
/// ```
/// PATCH
/// ```rust
/// # use annis::{Client, Method};
/// #
/// # fn patch() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let records = annis::me_records(Method::Patch, 1838569).params(vec![("rating", "5")]);
///
/// client.call(records)?;
/// # Ok(())
/// # }
/// ```

pub fn me_records(method: Method, id: usize) -> Service {
    let (client, params): (RequestBuilder, Option<Vec<(String, String)>>) = match method {
        Method::Post => (
            reqwest::Client::new().post("https://api.annict.com/v1/me/records"),
            Some(vec![("episodes_id".to_string(), id.to_string())]),
        ),
        Method::Patch => (
            reqwest::Client::new()
                .patch(format!("https://api.annict.com/v1/me/records/{}", id).as_str()),
            None,
        ),
        Method::Delete => (
            reqwest::Client::new()
                .delete(format!("https://api.annict.com/v1/me/records/{}", id).as_str()),
            None,
        ),
    };

    Service {
        client: client,
        params: params,
    }
}

/// Request to /v1/me/works
///
/// Examples
/// ========
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let me_works = annis::me_works().params(vec![("filter_title","機動戦士ガンダムUC")]);
///
/// client.call(me_works)?;
/// # Ok(())
/// # }
/// ```

pub fn me_works() -> Service {
    Service {
        client: reqwest::Client::new().get("https://api.annict.com/v1/me/works"),
        params: None,
    }
}

/// Request to /v1/me/programs
///
/// Examples
/// ========
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let episodes = annis::me_programs();
///
/// client.call(episodes)?;
/// # Ok(())
/// # }
/// ```

pub fn me_programs() -> Service {
    Service {
        client: reqwest::Client::new().get("https://api.annict.com/v1/me/programs"),
        params: None,
    }
}
