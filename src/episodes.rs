use reqwest::{Client};
use validate_sort_param;
use Nomalize;
use Service;

#[derive(Debug, Default)]
pub struct EpisodesBuilder {
    pub fields           : Option<String>,
    pub filter_ids       : Option<String>,
    pub filter_work_id   : Option<String>,
    pub page             : Option<String>,
    pub per_page         : Option<String>,
    pub sort_id          : Option<String>,
	pub sort_sort_number : Option<String>,
}

impl EpisodesBuilder {

	pub fn build(self) -> Service {
		let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(fields) = self.fields{
            params.push(("fields", fields));
        };
        if let Some(filter_ids) = self.filter_ids {
            params.push(("filter_ids", filter_ids));
        };
        if let Some(filter_work_id) = self.filter_work_id{
            params.push(("filter_work_id", filter_work_id));
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
        if let Some(sort_sort_number) = self.sort_sort_number {
            params.push(("sort_sort_number", sort_sort_number));
        };
		let params: Option<_> = if params == Vec::new()  {
            None
        }else {
            Some(params.iter().map(|(k, v)|{(k.to_string(), v.to_string())}).collect::<Vec<(String, String)>>())
        };
	    Service{
	    	client: Client::new().get("https://api.annict.com/v1/episodes"),
	    	params: params
	    }
	}

    pub fn fields<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        EpisodesBuilder{
            fields: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn filter_ids<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        EpisodesBuilder{
            filter_ids: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn filter_work_id<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        EpisodesBuilder{
            filter_work_id: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn page<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        EpisodesBuilder{
            page: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn per_page<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        EpisodesBuilder{
            per_page: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn sort_id<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        EpisodesBuilder{
            sort_id: Some(self::validate_sort_param(params.param_nomalizer())),
            ..self
        }
    }

    pub fn sort_sort_number<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        EpisodesBuilder{
            sort_sort_number: Some(self::validate_sort_param(params.param_nomalizer())),
            ..self
        }
    }
}	