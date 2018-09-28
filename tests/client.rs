extern crate annis;
use annis::{Client};
use std::env;

#[test]
fn set_token() {
	Client::set_token(env::var("annict_access_token").unwrap().to_string());
	Client::set_token(env::var("annict_access_token").unwrap());
}

#[test]
fn call() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let works = annis::works().params(vec![("filter_title", "lain")]);
	client.call(works).unwrap();
}
