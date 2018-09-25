[![Build Status](https://travis-ci.com/totechite/annict-rs.svg?branch=master)](https://travis-ci.com/totechite/annict-rs)   

annict-rs
==============
Annict API client library for Rust   

[Annict API Official Document]("https://docs.annict.com/")   

ToDo
-------------
- implement /v1/me/*. 
- Make clearly understandable to Function and Variable Names. 


Usage
--------------

Request to /v1/works.   
```rust
extern crate annis;
use annis::{Auth, Client};

fn main(){

	let auth = OAuth::client_id("client_id");
	let uri = &auth.authorize_url().redirect_uri("https://example.com").scope("read+write").build();

	// -> Browser access to this uri and Get a certification code.

	let access_token = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("certification code")
	.build();


	let client = Client::set_token(access_token);
	let works = annis::works().filter_title("lain").build();

	let json = client.call(works).unwrap();

	assert_eq!(json["works"][0]["title"], "serial experiments lain".to_string());
}
```

Auth Requests is two ways that instant or manual.   
```rust
extern crate annis;
use annis::{OAuth, AuthorizeUri, AccessToken};


	let auth = OAuth::client_id("client_id");

// Get Authorize URL
	let instant = auth.authorize_url();

	let manual = AuthorizeUri{
			client_id: "client_id".to_string(),
			redirect_uri: "urn:ietf:wg:oauth:2.0:oob".to_string(),
			scope: "read".to_string()
		}

	assert_eq!(instant, manual);


// Get AccessToken
	let instant = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("certification code")
	.build();

    let manual = AccessToken{
    		client_id: "client_id".to_string(),
    		client_secret: "client_secret_key"to_string(),
    		code: "certification code".to_string(),
    		redirect_uri: "urn:ietf:wg:oauth:2.0:oob".into()
    	}

	assert_eq!(instant, manual);

```

License
----------------------------
MIT license [http://opensource.org/licenses/mit-license.php]()