use reqwest::Client;
use Service;

#[derive(Debug, Default)]
pub struct MeRecords {
    pub params: Option<Vec<(String, String)>>
}

impl MeRecords {

    pub fn params<K, V>(self, params: Vec<(K, V)>) -> Self
     where K: Into<String>, V: Into<String>
    {
        let params: Vec<(String, String)> = params.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
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