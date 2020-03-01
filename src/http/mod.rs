use reqwest::{Response, Client};
use super::CfpError;

pub fn fetch(url: &str) -> Result<Response, CfpError> {
    let resp = Client::new()
        .get(url)
        .send()
        .unwrap();
    let status_code = resp.status().as_u16();

    match status_code {
        200 => return Ok(resp),
        400 => return Err(CfpError {
            kind: String::from("CfpTime"),
            message: "Undocumented".to_string(),
        }),
        _ => return Err(CfpError {
            kind: String::from("CfpTime"),
            message: "Unknown error".to_string(),
        }),
    }
}
