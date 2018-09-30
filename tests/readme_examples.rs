extern crate annis;
use annis::{OAuth, Client, AuthorizeUrl, AccessToken};
use std::env;

#[test]
fn main(){

	let auth = OAuth::client_id(env::var("annict_client_id").unwrap());
	let _url = &auth.authorize_url().redirect_uri("https://example.com").scope("read+write").build();

	// -> Browser access to this uri and Get a certification code.

	let _access_token = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("certification code")
	.build();


	let client = Client::set_token(env::var("annict_access_token").unwrap());
	let works = annis::works().params(vec![("filter_title", "lain")]);

	let _json = client.call(works).unwrap();

	// assert_eq!(json["works"][0]["title"], "serial experiments lain".to_string());
}

#[test]
fn auth_requests() {
	let auth = OAuth::client_id(env::var("annict_client_id").unwrap());

// Get Authorize URL
	let instant = auth.authorize_url().build();

	let manual = AuthorizeUrl{
			client_id: env::var("annict_client_id").unwrap(),
			redirect_uri: "urn:ietf:wg:oauth:2.0:oob".to_string(),
			scope: "read".to_string()
		}.build();

	assert_eq!(instant, manual);


// Get AccessToken
	let instant = auth
	.access_token()
	.client_secret("client_secret_key")
	.code("certification code")
	.build();

    let manual = AccessToken{
    		client_id: env::var("annict_client_id").unwrap(),
    		client_secret: "client_secret_key".to_string(),
    		code: "certification code".to_string(),
    		redirect_uri: "urn:ietf:wg:oauth:2.0:oob".into()
    	}.build();

	assert_eq!(instant, manual);
}