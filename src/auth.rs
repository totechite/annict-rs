use std::borrow::Cow;
use reqwest::{Client, Url};
use serde_json::{Value};

#[derive(Debug)]
pub struct Auth {
	client_id: Cow<'static, str>
}

#[derive(Debug)]
pub struct AuthUriBuilder {
	client_id: Cow<'static, str>,
	redirect_uri: Cow<'static, str>,
	scope: Cow<'static, str>
}

#[derive(Debug)]
pub struct TokenBuilder{
	client_id: Cow<'static, str>,
	client_secret: Cow<'static, str>,
	redirect_uri: Cow<'static, str>,
	code: Cow<'static, str>
}

impl Auth{

	pub fn client_id<T>(client_id: T) -> Auth
	where T: Into<Cow<'static, str>>
	{
		Auth{
			client_id: client_id.into()
		}
	}

	pub fn authorize(self) -> AuthUriBuilder{
		AuthUriBuilder::new(self.client_id)
	}

	pub fn token(self) -> TokenBuilder{
		TokenBuilder::new(self.client_id)
	}

}

impl AuthUriBuilder {

	fn new(client_id: Cow<'static, str>) -> Self{
		AuthUriBuilder{
			client_id: client_id.into(),
			redirect_uri: "urn:ietf:wg:oauth:2.0:oob".into(),
			scope: "read".into()
		}
	} 

	pub fn redirect_uri<T>(&mut self, redirect_uri: T) ->  &mut Self
		where T: Into<Cow<'static, str>> {
			self.redirect_uri = redirect_uri.into();
			self
	}

	pub fn scope<T>(&mut self, scope: T) ->  &mut Self
		where T: Into<Cow<'static, str>> {
			self.scope = scope.into();
			self
	}

	pub fn build(&self) -> String{
		format!("https://annict.com/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}",self.client_id, self.redirect_uri, self.scope)
	}

}

impl TokenBuilder {
    
    fn new(client_id: Cow<'static, str>) -> Self{
    	TokenBuilder{
    		client_id: client_id.into(),
    		client_secret: "".into(),
    		code: "".into(),
    		redirect_uri: "urn:ietf:wg:oauth:2.0:oob".into()
    	}
    }

	pub fn client_secret<T>(&mut self, client_secret: T) -> &mut Self
		where T: Into<Cow<'static, str>> {
			self.client_secret = client_secret.into();
			self
	}

	pub fn code<T>(&mut self, code: T) -> &mut Self
		where T: Into<Cow<'static, str>> {
			self.code = code.into();
			self
	}

	pub fn redirect_uri<T>(&mut self, redirect_uri: T) -> &mut Self
		where T: Into<Cow<'static, str>> {
			self.redirect_uri = redirect_uri.into();
			self
	}

	pub fn build(&self) -> String{
		let params: Vec<(&str, &str)> = vec![("client_id", &self.client_id), ("client_secret", &self.client_secret), ("grant_type", "authorization_code"),("redirect_uri", &self.redirect_uri), ("code", &self.code)];
	    let uri = Url::parse_with_params("https://api.annict.com/oauth/token",&params).unwrap();
	    Client::new()
	    .post(uri.as_str())
	    .form(&params)
	    .send().unwrap().json::<Value>().unwrap()["access_token"].to_string().trim_matches('\"').to_string()
	}
}