extern crate annis;
extern crate serde_json;
use annis::{Client};
use std::env;

fn main() -> Result<(), String>{

	let client = Client::set_token(
		env::var("annict_access_token").unwrap()
	);

	let params = vec![("filter_title", "lain"),("fields","title")];
    let works = annis::works().params(params).get();

	let json = client.call(works)?;
	
	println!("{:?}", json["works"]);

	Ok(())  
}