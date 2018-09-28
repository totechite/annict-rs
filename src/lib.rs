#![doc(html_root_url = "https://!docs.rs/annis/0.0.1")]

//! annis
//! =====
//! 
//! The `annis` is a Rust interface to the Annict API.
//! 
//! Request to /v1/works
//! -------------
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
mod client;
mod auth;

pub use serde_json::Value;
pub use client::Client;
pub use auth::*;


/// A Service to make request to endpoint.
///
///

#[derive(Debug)]
pub struct Service {
    pub client: RequestBuilder,
    pub params: Option<Vec<(String, String)>>
}

impl Service {

    pub fn params<K, V>(self, params: Vec<(K, V)>) -> Self
    	where K: Into<String>, V: Into<String>
    {
        let mut params: Vec<(String, String)> = params.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        if let Some(mut x) = self.params{
        	params.append(&mut x);
        };
        Self{
            params: Some(params),
            ..self
        }
    }

}

pub enum Methods{
	Create,
	Patch,
	Delete
}

pub fn works() -> Service {

	Service{
		client: reqwest::Client::new().get("https://api.annict.com/v1/works"),
		params: None
	}

}

pub fn episodes() -> Service {

	Service{
		client: reqwest::Client::new().get("https://api.annict.com/v1/episodes"),
		params: None
	}

}

pub fn records() -> Service {

	Service{
		client: reqwest::Client::new().get("https://api.annict.com/v1/records"),
		params: None
	}

}

pub fn me_statuses() -> Service {

	Service{
		client: reqwest::Client::new().post("https://api.annict.com/v1/me/statuses"),
		params: None
	}

}

pub fn me_records(method: Methods, id: usize) -> Service {

	let (client, params) = match method {
		Methods::Create => (reqwest::Client::new().post("https://api.annict.com/v1/me/records"), Some(vec![("episodes_id".to_string(), id.to_string())])),
		Methods::Patch  => (reqwest::Client::new().patch(format!("https://api.annict.com/v1/me/records/{}", id).as_str()), None),
		Methods::Delete => (reqwest::Client::new().delete(format!("https://api.annict.com/v1/me/records/{}", id).as_str()), None),
	};

	Service{
		client: client,
		params: params
	}

}

pub fn me_works() -> Service {

	Service{
		client: reqwest::Client::new().get("https://api.annict.com/v1/me/works"),
		params: None
	}

}

pub fn me_programs() -> Service {

	Service{
		client: reqwest::Client::new().get("https://api.annict.com/v1/me/programs"),
		params: None
	}

}
