extern crate annis;
use annis::Client;
use std::env;

#[test]
fn vannila_request() {
    let client = Client::set_token(env::var("annict_access_token").unwrap());
    let episodes = annis::episodes();
    client.call(episodes).unwrap();
}

#[test]
fn filter_work_id() {
    let client = Client::set_token(env::var("annict_access_token").unwrap());
    let episodes = annis::episodes().params(vec![("filter_work_id", "2274")]);
    let json = client.call(episodes).unwrap();
    assert_eq!(
        json["episodes"][0]["work"]["title"],
        "ゆゆ式".to_string()
    );
}
