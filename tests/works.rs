extern crate annis;
use annis::{Client, Value};
use std::env;

#[test]
fn vannila_request() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let works = annis::works().build();
	client.call(works).unwrap();
}

#[test]
fn filter_title() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let works = annis::works().filter_title("lain").build();
	let json = client.call(works).unwrap();
	assert_eq!(json["works"][0]["title"], "serial experiments lain".to_string())
}

#[test]
fn filter_ids() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let works = annis::works().filter_ids(860).build();
	let json = client.call(works).unwrap();
	assert_eq!(json["works"][0]["title"], "serial experiments lain".to_string());
	
	let client = Client::set_token(env::var("annict_access_token").unwrap());	
	let works = annis::works().filter_ids(0).build();
	let json = client.call(works).unwrap();
	assert_eq!(json["works"][0]["title"], Value::Null);
}