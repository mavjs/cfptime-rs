/*!
 * A rust library for interacting with the CFPTime API.
 *
 * For more information, the CFPTime API is documented at [api.cfptime.org/api/docs](https://api.cfptime.org/api/docs).
 *
 * Example:
 *
 * ```
 * use cfptime_api::{CFPTime, Conf};
 *
 * async fn get_all_cfps() {
 *     let cfptime = CFPTime::new();
 *
 *     let confs: Vec<Conf> = cfptime.get_cfps().await.unwrap();
 *     for conf in confs.clone().iter() {
 *         println!(
 *             "#{:?} - Name: {:?}, Country: {:?}, Website: {:?}",
 *             conf.id,
 *             conf.name,
 *             conf.country,
 *             conf.website
 *         );
 *     }
 * }
 * ```
 */
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

use reqwest::{Client, Method, Request, StatusCode, Url};
use serde::{Deserialize, Serialize};

// Endpoint for the CFPTime API.
const ENDPOINT: &str = "https://api.cfptime.org/api/";

// Entrypoint for interacting with the CFPTime API.
pub struct CFPTime {
    client: Arc<Client>,
}

impl CFPTime {
    /// Create a new CFPTime client struct.
    pub fn new() -> Self {
        let client = Client::builder()
            .build()
            .expect("creating client for CFPTime failed.");
        Self {
            client: Arc::new(client),
        }
    }

    fn request<B>(&self, method: Method, path: String, body: B) -> Request
    where
        B: Serialize,
    {
        let base_url = Url::parse(ENDPOINT).unwrap();
        let resource_url = base_url.join(&path).unwrap();

        let mut req = self.client.request(method.clone(), resource_url);

        if method != Method::GET && method != Method::DELETE {
            req = req.json(&body);
        }

        // Build the request.
        req.build().unwrap()
    }

    pub async fn get_cfps(&self) -> Result<Vec<Conf>, CFPError> {
        let request = self.request(Method::GET, "cfps".to_string(), ());

        let resp = self.client.execute(request).await.unwrap();
        match resp.status() {
            StatusCode::OK => (),
            s => {
                return Err(CFPError {
                    status_code: s,
                    body: resp.text().await.unwrap(),
                })
            }
        };

        let confs: Vec<Conf> = resp.json().await.unwrap();

        Ok(confs)
    }

    pub async fn get_cfp(&self, cfp_id: i32) -> Result<Conf, CFPError> {
        let request = self.request(
            Method::GET,
            format!("{}/{}/", "cfps".to_string(), cfp_id.to_string()),
            (),
        );

        let resp = self.client.execute(request).await.unwrap();
        match resp.status() {
            StatusCode::OK => (),
            s => {
                return Err(CFPError {
                    status_code: s,
                    body: resp.text().await.unwrap(),
                })
            }
        };

        let conf: Conf = resp.json().await.unwrap();

        Ok(conf)
    }

    pub async fn get_upcoming(&self) -> Result<Vec<Conf>, CFPError> {
        let request = self.request(Method::GET, "upcoming".to_string(), ());

        let resp = self.client.execute(request).await.unwrap();
        match resp.status() {
            StatusCode::OK => (),
            s => {
                return Err(CFPError {
                    status_code: s,
                    body: resp.text().await.unwrap(),
                })
            }
        };

        let upcoming: Vec<Conf> = resp.json().await.unwrap();

        Ok(upcoming)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub struct CFPError {
    pub status_code: StatusCode,
    pub body: String,
}

impl fmt::Display for CFPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CFPError: status code -> {}, body -> {}",
            self.status_code.to_string(),
            self.body
        )
    }
}

impl fmt::Debug for CFPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CFPError: status code -> {}, body -> {}",
            self.status_code.to_string(),
            self.body
        )
    }
}

// This is important for other errors to wrap this one.
impl error::Error for CFPError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
