extern crate annis;
use annis::{Client, Method};
use std::env;

#[test]
fn me_works() {
    let client = Client::set_token(env::var("annict_access_token").unwrap());
    let me_works = annis::me_works();
    client.call(me_works).unwrap();

    let client = Client::set_token(env::var("annict_access_token").unwrap());
    let me_works = annis::me_works().params(vec![("filter_title", "UC")]);
    let json = client.call(me_works).unwrap();
    assert_eq!(
        json["works"][0]["title"],
        "機動戦士ガンダムUC（ユニコーン） RE:0096".to_string()
    );
}

#[test]
fn me_programs() {
    let client = Client::set_token(env::var("annict_access_token").unwrap());
    let programs = annis::me_programs();
    client.call(programs).unwrap();
}

#[test]
fn me_statuses() {
    let client = Client::set_token(env::var("annict_access_token").unwrap());
    let statuses = annis::me_statuses().params(vec![("work_id", "3994"), ("kind", "watched")]);
    client.call(statuses).unwrap();
}

#[test]
fn me_records() {
    let client = Client::set_token(env::var("annict_access_token").unwrap());
    let records = annis::me_records(Method::Post, 5013).params(vec![("episode_id", "5013"), ("rating", "5")]);
    client.clone().call(records).unwrap();

    let records = annis::me_records(Method::Patch, 1838569).params(vec![("rating", "5")]);

    let json = client.call(records).unwrap();
    println!("{:?}", json);
}
