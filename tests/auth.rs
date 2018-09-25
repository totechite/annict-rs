extern crate annis;
use annis::OAuth;
use std::env;

#[test]
fn authorize() {
	OAuth::client_id(env::var("annict_client_id").unwrap())
	.authorize_url()
	.build();
}

