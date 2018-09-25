extern crate annis;
use annis::Auth;
use std::env;

#[test]
fn authorize() {
	Auth::client_id(env::var("annict_client_id").unwrap())
	.authorize()
	.build();
}
