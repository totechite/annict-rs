use reqwest::Client;
use Service;

#[derive(Debug, Default)]
pub struct MeRecords {
    pub params: Option<Vec<(String, String)>>
}
 impl MeRecords {

    pub fn params(self, params: Vec<(&str, &str)>) -> Self {
        let params: Vec<(String, String)> = params.iter().map(|(k, v)|{(k.to_string(), v.to_string())}).collect();
        Self{
            params: Some(params)
        }
    }

 	pub fn create(self) -> Service {
        Service{
            client: Client::new().post("https://api.annict.com/v1/me/records"),
            params: self.params
        }
 	}

    pub fn patch(self, id: usize) -> Service
    {   
        Service{
            client: Client::new().patch(format!("https://api.annict.com/v1/me/records/{}", id).as_str()),
            params: self.params
        }
    }

    pub fn detele(self, id: usize) -> Service
    {   
        Service{
            client: Client::new().patch(format!("https://api.annict.com/v1/me/records/{}", id).as_str()),
            params: None
        }
    }

 }