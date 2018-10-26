#![doc(html_root_url = "https://!docs.rs/annis/0.0.3")]

//! annis
//! =====
//!
//! The `annis` is a Rust interface to the Annict API.
//! [Annict API Official Document](https://docs.annict.com/)
//!
//! Usage
//! --------------------
//! Request to /v1/works
//! ```rust
//! # extern crate annis;
//! # use annis::{Client, Works};
//! # use std::env;
//! #
//! # fn main() -> Result<(), String>{
//! #
//! let client = Client::set_token("annict_access_token");
//!
//! let params = vec![(Works::filter_title, "lain"),(Works::fields,"title")];
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
use std::fmt;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate futures;

mod auth;
mod client;

pub use auth::*;
pub use client::Client;
pub use serde_json::Value;

/// A Service to make request to endpoint.   
///
///

#[derive(Debug)]
pub struct Service<P: Into<String> + std::cmp::PartialEq> {
    pub method: reqwest::Method,
    pub url: String,
    pub params: Option<Vec<(P, String)>>,
}

impl<P: Into<String> + std::cmp::PartialEq + IsValid> Service<P> {
    pub fn params<K, V>(self, params: Vec<(K, V)>) -> Service<P>
    where
        K: Into<P>,
        V: Into<String>,
    {
        let mut params: Vec<(P, String)> = params
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .filter(|(k, _)| k.is_valid())
            .collect();
        if let Some(mut x) = self.params {
            params.append(&mut x);
        };
        Service {
            params: Some(params),
            ..self
        }
    }
}

/// A type of argument for me_records().
pub enum Method {
    POST,
    PATCH,
    DELETE,
}

/// Request to /v1/works   
/// .params() assepts `Works` enum.
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
///
/// using enum code
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// use annis::Works::*;
///
/// let client = Client::set_token("annict_access_token");
///
/// let works = annis::works().params(vec![(filter_title, "lain")]);
///
/// client.call(works)?;
/// # Ok(())
/// # }
/// ```

pub fn works() -> Service<Works> {
    Service {
        method: reqwest::Method::GET,
        url: "https://api.annict.com/v1/works".to_string(),
        params: None,
    }
}

/// Request to /v1/episodes   
/// .params() assepts `Episodes` enum.
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
///
/// using enum code
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// use annis::Episodes::*;
///
/// let client = Client::set_token("annict_access_token");
///
/// let episodes = annis::episodes().params(vec![(filter_work_id, "2274")]);
///
/// client.call(episodes)?;
/// # Ok(())
/// # }
/// ```

pub fn episodes() -> Service<Episodes> {
    Service {
        method: reqwest::Method::GET,
        url: "https://api.annict.com/v1/episodes".to_string(),
        params: None,
    }
}

/// Request to /v1/records   
/// .params() assepts `Records` enum.
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
///
/// using enum code.
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// use annis::Records::*;
///
/// let client = Client::set_token("annict_access_token");
///
/// let records = annis::records().params(vec![(fields, "title")]);
///
/// client.call(records)?;
/// # Ok(())
/// # }
/// ```

pub fn records() -> Service<Records> {
    Service {
        method: reqwest::Method::GET,
        url: "https://api.annict.com/v1/records".to_string(),
        params: None,
    }
}

/// Request to /v1/me/statuses   
/// .params() assepts `MeStatuses` enum.
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
///
/// using enum code.
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// use annis::MeStatuses::*;
///
/// let client = Client::set_token("annict_access_token");
///
/// let statuses = annis::me_statuses().params(vec![(work_id, "3994"), (kind, "watched")]);
///
/// client.call(statuses)?;
/// # Ok(())
/// # }
/// ```

pub fn me_statuses() -> Service<MeStatuses> {
    Service {
        method: reqwest::Method::POST,
        url: "https://api.annict.com/v1/me/statuses".to_string(),
        params: None,
    }
}

/// Request to /v1/me/records   
/// .params() assepts `MeRecords` enum.
///
/// Examples
/// ========
///
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
///
/// using enum code
/// ****************
/// POST
/// ```rust
/// # use annis::{Client, Method};
/// #
/// # fn post() -> Result<(), String> {
/// use annis::MeRecords::*;
///
/// let client = Client::set_token("annict_access_token");
///
/// let records = annis::me_records(Method::Post, 5013).params(vec![(episode_id, "5013"), (rating, "5")]);
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
/// use annis::MeRecords::*;
///
/// let client = Client::set_token("annict_access_token");
///
/// let records = annis::me_records(Method::Patch, 1838569).params(vec![(rating, "5")]);
///
/// client.call(records)?;
/// # Ok(())
/// # }
/// ```

pub fn me_records(method: Method, id: usize) -> Service<MeRecords> {
    let (method, url, params): (reqwest::Method, String, Option<Vec<(MeRecords, String)>>) =
        match method {
            Method::POST => (
                reqwest::Method::POST,
                "https://api.annict.com/v1/me/records".to_string(),
                Some(vec![(MeRecords::episode_id, id.to_string())]),
            ),
            Method::PATCH => (
                reqwest::Method::PATCH,
                format!("https://api.annict.com/v1/me/records/{}", id),
                None,
            ),
            Method::DELETE => (
                reqwest::Method::DELETE,
                format!("https://api.annict.com/v1/me/records/{}", id),
                None,
            ),
        };

    Service {
        method: method,
        url: url,
        params: params,
    }
}

/// Request to /v1/me/works   
/// .params() assepts `MeWorks` enum.
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
/// using enum code
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// use annis::MeWorks::*;
///
/// let client = Client::set_token("annict_access_token");
///
/// let me_works = annis::me_works().params(vec![(filter_title,"機動戦士ガンダムUC")]);
///
/// client.call(me_works)?;
/// # Ok(())
/// # }
/// ```

pub fn me_works() -> Service<MeWorks> {
    Service {
        method: reqwest::Method::GET,
        url: "https://api.annict.com/v1/me/works".to_string(),
        params: None,
    }
}

/// Request to /v1/me/programs   
/// .params() assepts `MePrograms` enum.
///
/// Examples
/// ========
///
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// let client = Client::set_token("annict_access_token");
///
/// let programs = annis::me_programs().params(vec![("field", "id, title")]);
///
/// client.call(programs)?;
/// # Ok(())
/// # }
/// ```
/// using enum code
/// ```rust
/// # use annis::Client;
/// #
/// # fn run() -> Result<(), String> {
/// use annis::MePrograms::*;
///
/// let client = Client::set_token("annict_access_token");
///
/// let programs = annis::me_programs().params(vec![(fields, "id, title")]);
///
/// client.call(programs)?;
/// # Ok(())
/// # }
/// ```

pub fn me_programs() -> Service<MePrograms> {
    Service {
        method: reqwest::Method::GET,
        url: "https://api.annict.com/v1/me/programs".to_string(),
        params: None,
    }
}

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

/// used by works() function   
/// /v1/works assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    Invalid,
}

impl IsValid for Works {
    fn is_valid(&self) -> bool {
        *self != Works::Invalid
    }
}

impl From<Works> for String {
    fn from(p: Works) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Works {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(Works::Invalid)
    }
}

impl From<String> for Works {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Works::Invalid)
    }
}

impl fmt::Display for Works {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// used by episodes() function   
/// /v1/episodes assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Episodes {
    fields,
    filter_ids,
    filter_work_id,
    page,
    per_page,
    sort_id,
    sort_sort_number,
    Invalid,
}

impl IsValid for Episodes {
    fn is_valid(&self) -> bool {
        *self != Episodes::Invalid
    }
}

impl From<Episodes> for String {
    fn from(p: Episodes) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Episodes {
    fn from(p: &'static str) -> Episodes {
        serde_yaml::from_str(p).unwrap_or(Episodes::Invalid)
    }
}

impl From<String> for Episodes {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Episodes::Invalid)
    }
}

impl fmt::Display for Episodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// used by records() function   
/// /v1/records assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Records {
    fields,
    filter_ids,
    filter_episode_id,
    page,
    per_page,
    sort_id,
    sort_likes_count,
    Invalid,
}

impl IsValid for Records {
    fn is_valid(&self) -> bool {
        *self != Records::Invalid
    }
}

impl From<Records> for String {
    fn from(p: Records) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Records {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(Records::Invalid)
    }
}

impl From<String> for Records {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Records::Invalid)
    }
}

impl fmt::Display for Records {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// used by me_statuses() function   
/// /v1/me/statuses assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MeStatuses {
    work_id,
    kind,
    Invalid,
}

impl IsValid for MeStatuses {
    fn is_valid(&self) -> bool {
        *self != MeStatuses::Invalid
    }
}

impl From<MeStatuses> for String {
    fn from(p: MeStatuses) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MeStatuses {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(MeStatuses::Invalid)
    }
}

impl From<String> for MeStatuses {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(MeStatuses::Invalid)
    }
}

impl fmt::Display for MeStatuses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// used by me_records() function   
/// /v1/me/records assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MeRecords {
    episode_id,
    comment,
    rating,
    share_twitter,
    share_facebook,
    Invalid,
}

impl IsValid for MeRecords {
    fn is_valid(&self) -> bool {
        *self != MeRecords::Invalid
    }
}

impl From<MeRecords> for String {
    fn from(p: MeRecords) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MeRecords {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(MeRecords::Invalid)
    }
}

impl From<String> for MeRecords {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(MeRecords::Invalid)
    }
}

impl fmt::Display for MeRecords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// used by me_works() function   
/// /v1/me/works assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    Invalid,
}

impl IsValid for MeWorks {
    fn is_valid(&self) -> bool {
        *self != MeWorks::Invalid
    }
}

impl From<MeWorks> for String {
    fn from(p: MeWorks) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MeWorks {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(MeWorks::Invalid)
    }
}

impl From<String> for MeWorks {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(MeWorks::Invalid)
    }
}

impl fmt::Display for MeWorks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// used by me_programs() function   
/// /v1/me/programs assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    Invalid,
}

impl IsValid for MePrograms {
    fn is_valid(&self) -> bool {
        *self != MePrograms::Invalid
    }
}

impl From<MePrograms> for String {
    fn from(p: MePrograms) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MePrograms {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(MePrograms::Invalid)
    }
}

impl From<String> for MePrograms {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(MePrograms::Invalid)
    }
}

impl fmt::Display for MePrograms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
