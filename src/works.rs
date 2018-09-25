use reqwest::{Client};
use validete_season;
use validate_sort_param;
use Nomalize;
use Service;

#[derive(Debug, Default)]
pub struct WorksBuilder {
    pub fields              : Option<String>,
    pub filter_ids          : Option<String>,
    pub filter_season       : Option<String>,
    pub filter_title        : Option<String>,
    pub page                : Option<String>,
    pub per_page            : Option<String>,
    pub sort_id             : Option<String>,
    pub sort_season         : Option<String>,
    pub sort_watchers_count : Option<String>,
}

impl WorksBuilder{

    pub fn build(self) -> Service {
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(fields) = self.fields{
            params.push(("fields", fields));
        };
        if let Some(filter_ids) = self.filter_ids {
            params.push(("filter_ids", filter_ids));
        };
        if let Some(filter_season) = self.filter_season {
            params.push(("filter_season", filter_season));
        };
        if let Some(filter_title) = self.filter_title {
            params.push(("filter_title", filter_title));
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
        if let Some(sort_season) = self.sort_season {
            params.push(("sort_season", sort_season));
        };
        if let Some(sort_watchers_count) = self.sort_watchers_count {
            params.push(("sort_watchers_count", sort_watchers_count));
        };
        let params: Option<_> = if params == Vec::new()  {
            None
        }else {
            Some(params.iter().map(|(k, v)|{(k.to_string(), v.to_string())}).collect::<Vec<(String, String)>>())
        };
        Service{
            client: Client::new().get("https://api.annict.com/v1/works"),
            params: params
        }
    }

    pub fn fields<T>(self, params: T) -> Self
        where T: Nomalize
    {   
        WorksBuilder{
            fields: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn filter_ids<T>(self, params: T) -> Self 
        where T: Nomalize
    {
        WorksBuilder{
            filter_ids: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn filter_season<T>(self, params: T) -> Self 
        where T: Nomalize
    {
        ;
        WorksBuilder{
            filter_season: Some(self::validete_season(params.param_nomalizer())),
            ..self
        }
    }

    pub fn filter_title<T>(self, params: T) -> Self 
        where T: Nomalize
    {
        WorksBuilder{
            filter_title: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn page<T>(self, params: T) -> Self 
        where T: Nomalize
    {
        WorksBuilder{
            page: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn per_page<T>(self, params: T) -> Self 
        where T: Nomalize
    {
        WorksBuilder{
            per_page: Some(params.param_nomalizer()),
            ..self
        }
    }

    pub fn sort_id<T>(self, params: T) -> Self 
        where T: Nomalize
    {

        WorksBuilder{
            sort_id: Some(self::validate_sort_param(params.param_nomalizer())),
            ..self
        }
    }

    pub fn sort_season<T>(self, params: T) -> Self 
        where T: Nomalize
    {
        WorksBuilder{
            sort_season: Some(self::validate_sort_param(params.param_nomalizer())),
            ..self
        }
    }

    pub fn sort_watchers_count<T>(self, params: T) -> Self 
        where T: Nomalize
    {
        WorksBuilder{
            sort_watchers_count: Some(self::validate_sort_param(params.param_nomalizer())),
            ..self
        }
    }
}
