extern crate reqwest;
use reqwest::RequestBuilder;
extern crate serde_json;
pub mod client;
pub mod auth;
pub mod works;
pub mod episodes;
pub mod records;
pub mod me;
pub use serde_json::{Value};
pub use self::client::*;
pub use self::auth::*;
pub use self::works::*;
pub use self::episodes::*;
pub use self::records::*;
pub use self::me::statuses::*;
pub use self::me::works::*;
pub use self::me::records::*;
pub use self::me::programs::*;


#[derive(Debug)]
pub struct Service {
    pub client: RequestBuilder,
    pub params: Option<Vec<(String, String)>>
}

