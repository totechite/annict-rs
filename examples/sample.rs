extern crate annis;
use annis::{Client, Characters::*, Error, Value};
use std::env;

fn main() -> Result<(), Error> {
    let client = Client::set_token(env::var("annict_access_token").unwrap());

    let params = vec![(filter_ids, "26233"), (fields, "name")];
    let characters = annis::characters().params(params);
    let json = client.call(characters)?.json::<Value>()?;
    println!("{}", json["characters"][0]["name"].as_str().unwrap());

    Ok(())
}
