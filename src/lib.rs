#![doc(html_root_url = "https://!docs.rs/annis/0.0.4")]

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

mod auth;
mod client;

pub use crate::auth::*;
pub use crate::client::Client;
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

fn request<R: IsValid + Into<String> + std::cmp::PartialEq>(
    method: reqwest::Method,
    url: String,
) -> Service<R> {
    Service {
        method: method,
        url: url,
        params: None,
    }
}

/// Request to /v1/records   
/// .params() assepts `Records` enum.

pub fn reviews() -> Service<Reviews> {
    request(
        reqwest::Method::GET,
        "https://api.annict.com/v1/records".to_string(),
    )
}

/// used by records() function   
/// /v1/records assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Reviews {
    fields,
    filter_ids,
    filter_work_id,
    page,
    per_page,
    sort_id,
    sort_likes_count,
    Invalid,
}

impl IsValid for Reviews {
    fn is_valid(&self) -> bool {
        *self != Reviews::Invalid
    }
}

impl From<Reviews> for String {
    fn from(p: Reviews) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Reviews {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(Reviews::Invalid)
    }
}

impl From<String> for Reviews {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Reviews::Invalid)
    }
}

impl fmt::Display for Reviews {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Request to /v1/users   
/// .params() assepts `Users` enum.

pub fn users() -> Service<Users> {
    request(
        reqwest::Method::GET,
        "https://api.annict.com/v1/users".to_string(),
    )
}

/// used by users() function   
/// /v1/users assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Users {
    fields,
    filter_ids,
    filter_usernames,
    page,
    per_page,
    sort_id,
    Invalid,
}

impl IsValid for Users {
    fn is_valid(&self) -> bool {
        *self != Users::Invalid
    }
}

impl From<Users> for String {
    fn from(p: Users) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Users {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(Users::Invalid)
    }
}

impl From<String> for Users {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Users::Invalid)
    }
}

impl fmt::Display for Users {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Request to /v1/following   
/// .params() assepts `Following` enum.

pub fn following() -> Service<Following> {
    request(
        reqwest::Method::GET,
        "https://api.annict.com/v1/following".to_string(),
    )
}

/// used by following() function   
/// /v1/following assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Following {
    fields,
    filter_user_id,
    filter_username,
    page,
    per_page,
    sort_id,
    Invalid,
}

impl IsValid for Following {
    fn is_valid(&self) -> bool {
        *self != Following::Invalid
    }
}

impl From<Following> for String {
    fn from(p: Following) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Following {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(Following::Invalid)
    }
}

impl From<String> for Following {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Following::Invalid)
    }
}

impl fmt::Display for Following {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Request to /v1/followers   
/// .params() assepts `Followers` enum.

pub fn followers() -> Service<Followers> {
    request(
        reqwest::Method::GET,
        "https://api.annict.com/v1/followers".to_string(),
    )
}

/// used by followers() function   
/// /v1/followers assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Followers {
    fields,
    filter_user_id,
    filter_username,
    page,
    per_page,
    sort_id,
    Invalid,
}

impl IsValid for Followers {
    fn is_valid(&self) -> bool {
        *self != Followers::Invalid
    }
}

impl From<Followers> for String {
    fn from(p: Followers) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Followers {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(Followers::Invalid)
    }
}

impl From<String> for Followers {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Followers::Invalid)
    }
}

impl fmt::Display for Followers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
/// Request to /v1/activities   
/// .params() assepts `Activities` enum.

pub fn activities() -> Service<Activities> {
    request(
        reqwest::Method::GET,
        "https://api.annict.com/v1/activities".to_string(),
    )
}

/// used by activities() function   
/// /v1/activities assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Activities {
    fields,
    filter_users_ids,
    filter_username,
    page,
    per_page,
    sort_id,
    Invalid,
}

impl IsValid for Activities {
    fn is_valid(&self) -> bool {
        *self != Activities::Invalid
    }
}

impl From<Activities> for String {
    fn from(p: Activities) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Activities {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(Activities::Invalid)
    }
}

impl From<String> for Activities {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Activities::Invalid)
    }
}

impl fmt::Display for Activities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Request to /v1/me   
/// .params() assepts `Me` enum.

pub fn me() -> Service<Me> {
    request(
        reqwest::Method::GET,
        "https://api.annict.com/v1/me".to_string(),
    )
}

/// used by me() function   
/// /v1/me assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Me {
    fields,
    Invalid,
}

impl IsValid for Me {
    fn is_valid(&self) -> bool {
        *self != Me::Invalid
    }
}

impl From<Me> for String {
    fn from(p: Me) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for Me {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(Me::Invalid)
    }
}

impl From<String> for Me {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(Me::Invalid)
    }
}

impl fmt::Display for Me {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Request to /v1/me/reviews   
/// .params() assepts `MeReviews` enum.

// pub fn me_reviews() -> Service<MeReviews> {
//     request(
//         reqwest::Method::POST,
//         "https://api.annict.com/v1/me/reviews".to_string(),
//     )
// }
pub fn me_reviews(method: Method, id: usize) -> Service<MeReviews> {
        match method {
            Method::POST => {
                request(reqwest::Method::POST, "https://api.annict.com/v1/me/reviews".to_string(),)
                .params(vec![(MeReviews::work_id, id.to_string())])
            },
            Method::PATCH => request(
                reqwest::Method::PATCH,
                format!("https://api.annict.com/v1/me/reviews/{}", id),
            ),
            Method::DELETE => request(
                reqwest::Method::DELETE,
                format!("https://api.annict.com/v1/me/reviews/{}", id),
            ),
        }
}

/// used by me_reviews() function   
/// /v1/me/reviews assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MeReviews {
    work_id,
    title,
    body,
    rating_animation_state,
    rating_music_state,
    rating_story_state,
    rating_character_state,
    rating_overall_state,
    share_twitter,
    share_facebook,
    Invalid,
}

impl IsValid for MeReviews {
    fn is_valid(&self) -> bool {
        *self != MeReviews::Invalid
    }
}

impl From<MeReviews> for String {
    fn from(p: MeReviews) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MeReviews {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(MeReviews::Invalid)
    }
}

impl From<String> for MeReviews {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(MeReviews::Invalid)
    }
}

impl fmt::Display for MeReviews {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Request to /v1/me/following_activities   
/// .params() assepts `MeFollowing_activities` enum.

pub fn me_following_activities() -> Service<MeFollowing_activities> {
    request(
        reqwest::Method::GET,
        "https://api.annict.com/v1/me/following_activities".to_string(),
    )
}

/// used by me_following_activities() function   
/// /v1/me/following_activities assepts parameters.

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MeFollowing_activities {
    fields,
    filter_actions,
    filter_muted,
    page,
    per_page,
    sort_id,
    Invalid,
}

impl IsValid for MeFollowing_activities {
    fn is_valid(&self) -> bool {
        *self != MeFollowing_activities::Invalid
    }
}

impl From<MeFollowing_activities> for String {
    fn from(p: MeFollowing_activities) -> String {
        serde_json::to_string(&p).unwrap_or(String::from("invalid parameter"))
    }
}

impl From<&'static str> for MeFollowing_activities {
    fn from(p: &'static str) -> Self {
        serde_yaml::from_str(p).unwrap_or(MeFollowing_activities::Invalid)
    }
}

impl From<String> for MeFollowing_activities {
    fn from(p: String) -> Self {
        serde_yaml::from_str(p.as_str()).unwrap_or(MeFollowing_activities::Invalid)
    }
}

impl fmt::Display for MeFollowing_activities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
/// let records = annis::me_records(Method::POST, 5013).params(vec![("episode_id", "5013"), ("rating", "5")]);
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
/// let records = annis::me_records(Method::PATCH, 1838569).params(vec![("rating", "5")]);
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
/// let records = annis::me_records(Method::POST, 5013).params(vec![(episode_id, "5013"), (rating, "5")]);
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
/// let records = annis::me_records(Method::PATCH , 1838569).params(vec![(rating, "5")]);
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
