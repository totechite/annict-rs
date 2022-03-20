# annict-rs

[![Build Status](https://travis-ci.com/totechite/annict-rs.svg?branch=master)](https://travis-ci.com/totechite/annict-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/f39tjurl4m7ggkch/branch/master?svg=true)](https://ci.appveyor.com/project/totechite/annict-rs/branch/master)
[![crates.io](https://img.shields.io/crates/v/annis.svg)](https://crates.io/crates/annis)

English: [README.md](./README.md)

Annict API の Rust ライブラリです

- [Annict API 公式ドキュメント](https://docs.annict.com/)
- [Library Document](https://docs.rs/annis)
- [Changelog](https://github.com/totechite/annict-rs/blob/master/CHANGELOG.md)

## インストール

Cargo.toml に追記

```toml
[dependencies]
annis = "0.0.6"
```

## 使い方＆仕様例

[Annict API 公式ドキュメント](https://docs.annict.com/)や[ライブラリのドキュメント](https://docs.rs/annis)を参考にしてください。

以下は認証を行ったのち、/v1/works にリクエストを送るコードです。  

・非同期リクエスト
```rust
use annis::nonblocking::Client;
use annis::{ OAuth, Value, Error };
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {

	let auth = OAuth::client_id("client_id");
	let url = auth.authorize_url().redirect_uri("https://example.com").scope("read+write").build();

	// 認証コードを取得

	let access_token = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("認証コード")
	.build();

        // 取得したアクセストークンを用いて/v1/worksにリクエストを送信
	let client = Client::set_token(access_token);
	let works = annis::works().params(vec![("filter_title", "CENCOROLL")]);

	let json = client.call(works).await?.json::<Value>().await?;

	assert_eq!(json["works"][0]["title"], "CENCOROLL -センコロール-".to_string());

    Ok(())
}
```

・同期リクエスト
```rust
use annis::{OAuth, Client , Value, Error };

fn main() -> Result<(), Error> {

	let auth = OAuth::client_id("client_id");
	let url = auth.authorize_url().redirect_uri("https://example.com").scope("read+write").build();

	// 認証コードを取得

	let access_token = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("認証コード")
	.build();

    // 取得したアクセストークンを用いて/v1/worksにリクエストを送信
	let client = Client::set_token(access_token);
	let works = annis::works().params(vec![("filter_title", "CENCOROLL")]);

	let json = client.call(works)?.json::<Value>()?;

	assert_eq!(json["works"][0]["title"], "CENCOROLL -センコロール-".to_string());

    Ok(())
}
```

以下はアクセストークンを取得するコードです。
```rust
use annis::{OAuth, AuthorizeUrl, AccessToken};
// 非同期リクエストの場合はnonblockingモジュールを使用してください。
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
	.build().await;

    let manual = AccessToken{
    		client_id: "client_id".to_string(),
    		client_secret: "client_secret_key".to_string(),
    		code: "certification code".to_string(),
    		redirect_uri: "urn:ietf:wg:oauth:2.0:oob".into()
    	}.build();

	assert_eq!(instant, manual);
```
