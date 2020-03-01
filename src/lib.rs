#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

pub mod cfps;
pub mod http;

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub id: i32,
    pub name: String,
    pub cfp_deadline: String,
    pub conf_start_date: String,
    pub city: String,
    pub province: String,
    pub country: String,
    pub twitter: String,
    pub website: String,
    pub cfp_details: String,
    pub speaker_benefits: String,
    pub code_of_conduct: String,
    pub created_at: String,
    pub number_of_days: i32,
}

#[derive(Copy, Clone)]
pub struct CfpTime<'a> {
    endpoint: &'a str
}

impl <'a>CfpTime<'a> {
    pub fn new() -> Self {
        CfpTime {
            endpoint: "https://api.cfptime.org/api"
        }
    }
}

#[derive(Debug)]
pub struct CfpError {
    pub kind: String,
    pub message: String
}

impl From<reqwest::Error> for CfpError {
    fn from(error: reqwest::Error) -> Self {
        CfpError {
            kind: String::from("reqwest"),
            message: error.to_string(),
        }
    }
}
