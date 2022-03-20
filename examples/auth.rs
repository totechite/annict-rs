use annis::nonblocking::{Client, OAuth, AuthorizeUrl, AccessToken};
use annis::{Error, Value};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth = OAuth::client_id(env::var("annict_client_id").unwrap());
    let url = auth
        .authorize_url()
        .redirect_uri("urn:ietf:wg:oauth:2.0:oob")
        .scope("read+write")
        .build();

    // 認証コードを取得
    println!("{}", url);

    let access_token = auth
        .access_token()
        .client_secret("client_secret_key")
        .code("認証コード")
        .build().await;

    // 取得したアクセストークンを用いて/v1/worksにリクエストを送信
    let client = Client::set_token(access_token);
    let works = annis::works().params(vec![("filter_title", "lain")]);

    let json = client.call(works).await?.json::<Value>().await?;

    assert_eq!(
        json["works"][0]["title"],
        "serial experiments lain".to_string()
    );

    Ok(())
}
