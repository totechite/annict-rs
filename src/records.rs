use reqwest::Client;
use Service;

#[derive(Debug, Default)]
pub struct Records {
    pub params: Option<Vec<(String, String)>>
}

impl Records {

    pub fn params<K, V>(self, params: Vec<(K, V)>) -> Self
     where K: Into<String>, V: Into<String>
    {
        let params: Vec<(String, String)> = params.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Self{
            params: Some(params)
        }
    }

    pub fn get(self) -> Service {
    	Service{
    		client: Client::new().get("https://api.annict.com/v1/records"),
    		params: self.params
    	}
    }
}