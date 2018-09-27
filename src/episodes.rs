use reqwest::Client;
use Service;

#[derive(Debug, Default)]
pub struct Episodes {
    pub params: Option<Vec<(String, String)>>
}

impl Episodes {

    pub fn params(self, params: Vec<(&str, &str)>) -> Self 
    {
        let params: Vec<(String, String)> = params.iter().map(|(k, v)|{(k.to_string(), v.to_string())}).collect();
        Self{
            params: Some(params)
        }
    }

	pub fn get(self) -> Service {

	    Service{
	    	client: Client::new().get("https://api.annict.com/v1/episodes"),
	    	params: self.params
	    }
	}

}	