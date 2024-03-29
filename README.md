# annict-rs

[![Build Status](https://travis-ci.com/totechite/annict-rs.svg?branch=master)](https://travis-ci.com/totechite/annict-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/f39tjurl4m7ggkch/branch/master?svg=true)](https://ci.appveyor.com/project/totechite/annict-rs/branch/master)
[![crates.io](https://img.shields.io/crates/v/annis.svg)](https://crates.io/crates/annis)

日本語: [README-ja.md](./README-ja.md)

Annict API client library for Rust.

- [Annict API Official Document](https://docs.annict.com/)
- [Library Document](https://docs.rs/annis)
- [Changelog](https://github.com/totechite/annict-rs/blob/master/CHANGELOG.md)

Annict is a web service to make and manage watching anime life.

- [Annict](https://annict.com)
- [Annict's Github Account](https://github.com/annict)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
annis = "0.0.6"
```

and this to your crate root:

```rust
extern crate annis;
```

Here is a example code that the process from obtaining `access_token` until making a request for /v1/works.

・Asynchronous example

```rust
use annis::nonblocking::Client;
use annis::{ OAuth, Value, Error };
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {

	let auth = OAuth::client_id("client_id");
	let url = auth.authorize_url().redirect_uri("https://example.com").scope("read+write").build();

	// Get a certification code.

	let access_token = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("認証コード")
	.build();

	let client = Client::set_token(access_token);
	let works = annis::works().params(vec![("filter_title", "lain")]);

	let json = client.call(works).await?.json::<Value>().await?;

	assert_eq!(json["works"][0]["title"], "serial experiments lain".to_string());

    Ok(())
}
```

・Synchronous example

```rust
use annis::{OAuth, Client , Value, Error };

fn main() -> Result<(), Error> {

	let auth = OAuth::client_id("client_id");
	let url = auth.authorize_url().redirect_uri("https://example.com").scope("read+write").build();

	// Get a certification code.

	let access_token = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("認証コード")
	.build();

	let client = Client::set_token(access_token);
	let works = annis::works().params(vec![("filter_title", "lain")]);

	let json = client.call(works)?.json::<Value>()?;

	assert_eq!(json["works"][0]["title"], "serial experiments lain".to_string());

    Ok(())
}
```

Auth Requests were made two ways that methods or creating struct.

```rust
use annis::{OAuth, AuthorizeUri, AccessToken};
// Please use nonblocking module in case of asynchronous request.
// use annis::nonblocking::{OAuth, AuthorizeUrl, AccessToken};

	let auth = OAuth::client_id("client_id");

// Get Authorize URL
	let instant = auth.authorize_url().build();

	let manual = AuthorizeUrl{
			client_id: "client_id".to_string(),
			redirect_uri: "urn:ietf:wg:oauth:2.0:oob".to_string(),
			scope: "read".to_string()
		}.build();

	assert_eq!(instant, manual);


// Get AccessToken
	let instant = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("certification code")
	.build();

    let manual = AccessToken{
    		client_id: "client_id".to_string(),
    		client_secret: "client_secret_key".to_string(),
    		code: "certification code".to_string(),
    		redirect_uri: "urn:ietf:wg:oauth:2.0:oob".into()
    	}.build();

	assert_eq!(instant, manual);
```

Parameter arguments accept &str, String and Enum.

```rust
extern crate annis;
use annis::{OAuth, Client, Works::*};

	let use_enum = annis::works().params(vec![(filter_title, "lain")]);
	let use_string = annis::works().params(vec![("filter_title", "lain")]);

	assert_eq!(use_enum.params, use_string.params);

```

## License

MIT license  
[http://opensource.org/licenses/mit-license.php]()
