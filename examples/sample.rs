extern crate annis;
use annis::{Client, Works::*, Error};
use std::env;

fn main() -> Result<(), Error> {
    let client = Client::set_token(env::var("annict_access_token").unwrap());

    let params = vec![(filter_title, "lain"), (fields, "title")];
    let works = annis::works().params(params);
    let json = client.call(works)?;
    println!("{:?}", json);

    Ok(())
}
