extern crate annis;
extern crate serde_json;
use annis::{Client};
use std::env;

fn main() -> Result<(), String>{
	let client = Client::set_token(
		env::var("annict_access_token").unwrap()
	);
    let works = annis::works().filter_title("lain").build();
	let result = client.call(works)?;
	println!("{:?}", result["works"]);
	Ok(())  
}