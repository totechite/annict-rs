use works::*;
use episodes::*;
use records::*;
use std::borrow::Cow;
use Value;
use Service;

#[derive(Debug)]
pub struct Client {
    pub token: Cow<'static, str>
}

impl Client {

	pub fn set_token<T>(access_token: T) -> Self
	where T: Into<Cow<'static, str>>
	{
		Client{
			token: access_token.into()
		}
	}

	pub fn call(self, service: Service) -> Result<Value, String>
    {
    	let mut client = service.client
        .bearer_auth(self.token);
        if let Some(params) = service.params {
        	client = client.query(&params);
        };
        let mut req = try!(client.send().map_err(|err| err.to_string()));
        req.json::<Value>().or(Err("Invalid values at token or request parameters".to_string()))
    }
}

pub fn works() -> WorksBuilder {
	WorksBuilder::default()
}

pub fn episodes() -> EpisodesBuilder {
    EpisodesBuilder::default()
}

pub fn records() -> RecordsBuilder {
	RecordsBuilder::default()
}

