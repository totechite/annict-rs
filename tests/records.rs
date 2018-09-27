extern crate annis;
use annis::Client;
use std::env;

#[test]
fn vannila_request() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let records = annis::records().get();
	client.call(records).unwrap();	
}

#[test]
fn fields() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let records = annis::records().params(vec![("fields", "title")]).get();
	client.call(records).unwrap();
}
