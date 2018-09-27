extern crate annis;
extern crate serde_json;
use annis::{Client};
use std::env;

fn main() -> Result<(), String>{
	let client = Client::set_token(
		env::var("annict_access_token").unwrap()
	);
    let works = annis::works().params(vec![("filter_title", "lain"),("fields","title")]).get();
	let result = client.call(works)?;
	println!("{:?}", result["works"]);

	let records = annis::me_records().params(vec![("episode_id", "5013"), ("rating", "5")]).create();
	let json = client.call(records).unwrap();
	println!("{:?}", json);

	Ok(())  
}