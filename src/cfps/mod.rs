use reqwest::Client;
use serde_json::from_str;
use super::{http::fetch, CfpTime, Conf, CfpError};


impl <'a>CfpTime<'a> {
    /// Get a list of all the upcoming Call for Papers.
    ///
    /// # Example
    ///```
    /// use cfptime::*;
    ///
    /// let cfps = CfpTime::new();
    /// cfps.get_cfps();
    ///```
    pub fn get_cfps(self) -> Result<Vec<Conf>, CfpError> {
        let url = format!("{}/cfps/", &self.endpoint);
        let mut resp = fetch(&url)?;
        let body: &str = &resp.text().unwrap();
        let conf: Vec<Conf> = from_str(&body).unwrap();

        Ok(conf)
    }

    /// Get an upcoming Call for Papers by ID.
    ///
    /// # Example
    ///```
    /// use cfptime::*;
    ///
    /// let cfps = CfpTime::new();
    /// cfps.get_cfp_id(1038i32);
    ///```
    pub fn get_cfp_id(self, id: i32) -> Conf {
        //let url = &[self.endpoint, "cfps", &id.to_string()].join("/");
        let url = format!("{}/cfps/{}", self.endpoint, &id.to_string());
        let mut resp = Client::new()
            .get(&url)
            .send()
            .unwrap();
        let body: &str = &resp.text().unwrap();
        from_str(&body).unwrap()
    }
}
