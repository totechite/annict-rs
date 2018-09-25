use reqwest::{Client};
use validate_sort_param;
use Nomalize;
use Service;

#[derive(Debug, Default)]
pub struct RecordsBuilder {
	pub fields           	: Option<String>,
	pub filter_ids       	: Option<String>,
	pub filter_episode_id   : Option<String>,
	pub page             	: Option<String>,
	pub per_page         	: Option<String>,
	pub sort_id          	: Option<String>,
	pub sort_likes_count 	: Option<String>,
}

impl RecordsBuilder {
    
    pub fn build(self) -> Service {
		let mut params: Vec<(&str, String)> = Vec::new();
		
        if let Some(fields) = self.fields{
            params.push(("fields", fields));
        };
        if let Some(filter_ids) = self.filter_ids {
            params.push(("filter_ids", filter_ids));
        };
        if let Some(filter_episode_id) = self.filter_episode_id{
            params.push(("filter_episode_id", filter_episode_id));
        };
        if let Some(page) = self.page {
            params.push(("page", page));
        };
        if let Some(per_page) = self.per_page {
            params.push(("per_page", per_page));
        };
        if let Some(sort_id) = self.sort_id {
            params.push(("sort_id", sort_id));
        };
        if let Some(sort_likes_count) = self.sort_likes_count {
            params.push(("sort_likes_count", sort_likes_count));
        };
		let params: Option<_> = if params == Vec::new()  {
            None
        }else {
            Some(params.iter().map(|(k, v)|{(k.to_string(), v.to_string())}).collect::<Vec<(String, String)>>())
        };
    	Service{
    		client: Client::new().get("https://api.annict.com/v1/records"),
    		params: params
    	}
    }

    pub fn fields<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        RecordsBuilder{
            fields: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn filter_ids<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        RecordsBuilder{
            filter_ids: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn filter_episode_id<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        RecordsBuilder{
            filter_episode_id: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn page<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        RecordsBuilder{
            page: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn per_page<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        RecordsBuilder{
            per_page: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn sort_id<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        RecordsBuilder{
            sort_id: Some(self::validate_sort_param(params.param_nomalizer())),
            ..self
        }
    }

    pub fn sort_likes_count<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        RecordsBuilder{
            sort_likes_count: Some(self::validate_sort_param(params.param_nomalizer())),
            ..self
        }
    }
}