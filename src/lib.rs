extern crate reqwest;
use reqwest::RequestBuilder;
extern crate serde_json;
pub mod client;
pub mod auth;
pub mod works;
pub mod episodes;
pub mod records;
pub use serde_json::{Value};
pub use self::client::*;
pub use self::auth::*;
pub use self::works::*;
pub use self::episodes::*;
pub use self::records::*;

#[derive(Debug)]
pub struct Service {
    pub client: RequestBuilder,
    pub params: Option<Vec<(String, String)>>
}

pub trait Nomalize {
    fn param_nomalizer(self) -> String;
}

impl Nomalize for Vec<&'static str> {
    fn param_nomalizer(self) -> String {
        self.join(",")
    }
}

impl Nomalize for String {
    fn param_nomalizer(self) -> String {
        self
    }
}

impl Nomalize for &'static str {
    fn param_nomalizer(self) -> String {
        self.to_string()
    }
}

impl Nomalize for Vec<usize>{
    fn param_nomalizer(self) -> String {
        self.iter().map(|x| {x.to_string()}).collect::<Vec<String>>().join(",")
    }
}

impl Nomalize for usize{
    fn param_nomalizer(self) -> String {
        self.to_string()
    }
}


pub fn validate_sort_param(param: String) -> String{
    match param.as_str(){
        "asc" | "desc" => param,
        _ => panic!("{} is invalid value. Please select a collect value from \"asc\" or \"desc\"", param)
    }
}

pub fn validete_season(param: String) -> String{
    match param.split("-").collect::<Vec<&str>>()[1] {
       "all" | "spring" | "summar" | "autumn" | "winter" => param,
        _ => panic!("{} is invalid value. Please select a season. ex. \"{}-spring\"", param, param.split("-").collect::<Vec<&str>>()[0]),
    }
}