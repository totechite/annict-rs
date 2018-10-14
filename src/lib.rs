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
use std::fmt;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

mod auth;
mod client;

pub use auth::*;
pub use client::Client;
pub use serde_json::Value;

/// A Service to make request to endpoint.
///
///

#[derive(Debug)]
pub struct Service<P> {
    pub client: RequestBuilder,
    pub params: Option<Vec<(P, String)>>,
}

impl<P: Into<String>> Service<P> {
    pub fn params<K, V>(self, params: Vec<(K, V)>) -> Service<P>
    where
        K: Into<P>,
        V: Into<String>,
    {
        let mut params: Vec<(P, String)> = params
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        if let Some(mut x) = self.params {
            params.append(&mut x);
        };
        Service {
            client: self.client,
            params: Some(params),
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

pub fn works() -> Service<Works> {
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

pub fn episodes() -> Service<Episodes> {
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

pub fn records() -> Service<Records> {
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

pub fn me_statuses() -> Service<MeStatuses> {
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

pub fn me_records(method: Method, id: usize) -> Service<MeRecords> {
    let (client, params): (RequestBuilder, Option<Vec<(MeRecords, String)>>) = match method {
        Method::Post => (
            reqwest::Client::new().post("https://api.annict.com/v1/me/records"),
            Some(vec![(MeRecords::episode_id, id.to_string())]),
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

pub fn me_works() -> Service<MeWorks> {
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
/// let programs = annis::me_programs();
///
/// client.call(programs)?;
/// # Ok(())
/// # }
/// ```

pub fn me_programs() -> Service<MePrograms> {
    Service {
        client: reqwest::Client::new().get("https://api.annict.com/v1/me/programs"),
        params: None,
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Works {
    fields,
    filter_ids,
    filter_season,
    filter_title,
    page,
    per_page,
    sort_id,
    sort_season,
    sort_watchers_count,
}

impl From<Works> for String {
    fn from(p: Works) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Works {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).expect("err")
    }
}

impl fmt::Display for Works {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Episodes {
    fields,
    filter_ids,
    filter_work_id,
    page,
    per_page,
    sort_id,
    sort_sort_number,
}

impl From<Episodes> for String {
    fn from(p: Episodes) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Episodes {
    fn from(p: &'static str) -> Episodes {
        serde_yaml::from_str(p).expect("err")
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Records {
    fields,
    filter_ids,
    filter_episode_id,
    page,
    per_page,
    sort_id,
    sort_likes_count,
}

impl From<Records> for String {
    fn from(p: Records) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Records {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).expect("err")
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MeStatuses {
    work_id,
    kind,
}

impl From<MeStatuses> for String {
    fn from(p: MeStatuses) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MeStatuses {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).expect("err")
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MeRecords {
    episode_id,
    comment,
    rating,
    share_twitter,
    share_facebook,
}

impl From<MeRecords> for String {
    fn from(p: MeRecords) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MeRecords {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).expect("err")
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MeWorks {
    fields,
    filter_ids,
    filter_season,
    filter_title,
    filter_status,
    page,
    per_page,
    sort_id,
    sort_season,
    sort_watchers_count,
}

impl From<MeWorks> for String {
    fn from(p: MeWorks) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MeWorks {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MePrograms {
    fields,
    filter_ids,
    filter_channel_ids,
    filter_work_ids,
    filter_started_at_gt,
    filter_started_at_lt,
    filter_unwatched,
    filter_rebroadcast,
    page,
    per_page,
    sort_id,
    sort_started_at,
}

impl From<MePrograms> for String {
    fn from(p: MePrograms) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MePrograms {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap()
    }
}
