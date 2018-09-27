use works::Works;
use episodes::Episodes;
use records::Records;
pub use me::statuses::MeStatuses;
pub use me::works::MeWorks;
pub use me::records::MeRecords;
pub use me::programs::MePrograms;
use Value;
use Service;

#[derive(Debug, Clone)]
pub struct Client {
    pub token: String
}

impl Client {

	pub fn set_token<T>(access_token: T) -> Self
	where T: Into<String>
	{
		Client{
			token: access_token.into()
		}
	}

	pub fn call(&self, service: Service) -> Result<Value, String>
    {
    	let mut client = service.client
        .bearer_auth(self.clone().token);
        if let Some(params) = service.params {
        	client = client.query(&params);
        };
        let mut req = try!(client.send().map_err(|err| err.to_string()).or(Err("Invalid values at token or request parameters".to_string())));
        req.json::<Value>().or(Ok(Value::Null))
    }
}

pub fn works() -> Works {
	Works::default()
}

pub fn episodes() -> Episodes {
    Episodes::default()
}

pub fn records() -> Records {
	Records::default()
}

pub fn me_statuses() -> MeStatuses {
    MeStatuses::default()
}

pub fn me_records() -> MeRecords {
    MeRecords::default()
}

pub fn me_works() -> MeWorks {
    MeWorks::default()
}

pub fn me_programs() -> MePrograms {
    MePrograms::default()
}