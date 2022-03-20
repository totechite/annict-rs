extern crate annis;
use annis::nonblocking::Client;
use annis::{Characters::*, Error, Value};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::set_token(env::var("annict_access_token").unwrap());

    let params = vec![(filter_ids, "26233"), (fields, "name")];
    let characters = annis::characters().params(params);
    let json = client.call(characters).await?.json::<Value>().await?;
    println!("{}", json["characters"][0]["name"].as_str().unwrap());

    Ok(())
}
