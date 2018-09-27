use std::borrow::Cow;
use reqwest::{Client, Url};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct OAuth {
	pub client_id: String
}

#[derive(Debug, PartialEq)]
pub struct AuthorizeUrl{
	pub client_id: String,
	pub redirect_uri: String,
	pub scope: String
}

#[derive(Debug, PartialEq)]
pub struct AccessToken{
	pub client_id: String,
	pub client_secret: String,
	pub redirect_uri: String,
	pub code: String
}

impl OAuth{

	pub fn client_id<P>(client_id: P) -> OAuth
		where P: Into<Cow<'static, str>>
	{
		OAuth{
			client_id: client_id.into().to_string()
		}
	}

	pub fn authorize_url(&self) -> AuthorizeUrl{
		AuthorizeUrl::new(self.clone().client_id)
	}

	pub fn access_token(&self) -> AccessToken{
		AccessToken::new(self.clone().client_id)
	}

}

impl AuthorizeUrl{

	fn new(client_id: String) -> Self{
		AuthorizeUrl{
			client_id: client_id,
			redirect_uri: "urn:ietf:wg:oauth:2.0:oob".into(),
			scope: "read".into()
		}
	} 

	pub fn redirect_uri<P>(&mut self, redirect_uri: P) ->  &mut Self
		where P: Into<Cow<'static, str>>
	{
		self.redirect_uri = redirect_uri.into().to_string();
		self
	}

	pub fn scope<P>(&mut self, scope: P) ->  &mut Self
		where P: Into<Cow<'static, str>>
	{
		self.scope = scope.into().to_string();
		self
	}

	pub fn build(&self) -> String{
		format!("https://annict.com/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}",self.client_id, self.redirect_uri, self.scope)
	}

}

impl AccessToken {
    
    fn new(client_id: String) -> Self{
    	AccessToken{
    		client_id: client_id.into(),
    		client_secret: "".into(),
    		code: "".into(),
    		redirect_uri: "urn:ietf:wg:oauth:2.0:oob".into()
    	}
    }

	pub fn client_secret<P>(&mut self, client_secret: P) -> &mut Self
		where P: Into<Cow<'static, str>>
	{
		self.client_secret = client_secret.into().to_string();
		self
	}

	pub fn code<P>(&mut self, code: P) -> &mut Self
		where P: Into<Cow<'static, str>>
	{
		self.code = code.into().to_string();
		self
	}

	pub fn redirect_uri<P>(&mut self, redirect_uri: P) -> &mut Self
		where P: Into<Cow<'static, str>>
	{
		self.redirect_uri = redirect_uri.into().to_string();
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