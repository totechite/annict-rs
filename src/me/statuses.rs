use reqwest::Client;
use Service;

#[derive(Debug, Default)]
pub struct MeStatuses {
    pub params: Option<Vec<(String, String)>>
}

impl MeStatuses {

    pub fn params(self, params: Vec<(&str, &str)>) -> Self 
    {
        let params: Vec<(String, String)> = params.iter().map(|(k, v)|{(k.to_string(), v.to_string())}).collect();
        Self{
            params: Some(params)
        }
    }

	pub fn create(self) -> Service{
        Service{
			client: Client::new().post("https://api.annict.com/v1/me/statuses"),
			params: self.params
		}
	}
}