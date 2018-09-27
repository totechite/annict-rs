extern crate annis;
use annis::Client;
use std::env;

#[test]
fn me_works() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let me_works = annis::me_works().get();
	client.call(me_works).unwrap();

	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let me_works = annis::me_works().params(vec![("filter_title","UC")]).get();
	let json = client.call(me_works).unwrap();
	assert_eq!(json["works"][0]["title"] , "機動戦士ガンダムUC（ユニコーン） RE:0096".to_string());
}

#[test]
fn me_programs() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let episodes = annis::me_programs().get();
	client.call(episodes).unwrap();	
}

#[test]
fn me_statuses() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let statuses = annis::me_statuses().params(vec![("work_id", "3994"), ("kind", "watched")]).create();
	client.call(statuses).unwrap();	
}

#[test]
fn me_records() {
	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let records = annis::me_records().params(vec![("episode_id", "5013"), ("rating", "5")]).create();
	client.clone().call(records).unwrap();

	let records = annis::me_records().params(vec![("rating", "5")]).patch(1838569);
	let json = client.call(records).unwrap();
	println!("{:?}", json);

}