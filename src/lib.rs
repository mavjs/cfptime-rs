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


use anyhow::{Result, bail};
use reqwest::{header, Method, Request, Url, StatusCode};
use serde::{Serialize, Deserialize};

// Endpoint for the CFPTime API.
const ENDPOINT: &str = "https://api.cfptime.org/api/";

// Entrypoint for interacting with the CFPTime API.
pub struct CFPTime {
    pub(crate) http_client: reqwest_middleware::ClientWithMiddleware,
}

impl CFPTime {
    /// Create a new CFPTime client struct.
    pub fn new() -> Self {
        let http = reqwest::Client::builder().build();
        match http {
            Ok(lclient) => {
                let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
                let client = reqwest_middleware::ClientBuilder::new(lclient)
                .with(reqwest_tracing::TracingMiddleware::default())
                .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy))
                .build();

                Self {
                    http_client: client,
                }
            }
            Err(err) => panic!("creating client failed: {err:?}"),
        }
    }

    pub(crate) fn request<B>(
        &self,
        method: Method,
        path: String,
        body: B,
    ) -> Result<Request>
    where
        B: Serialize,
    {
        let base = Url::parse(ENDPOINT)?;
        let url = base.join(&path)?;

        // Set the default headers.
        let mut headers = header::HeaderMap::new();
        headers.append(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json; charset=utf-8"),
        );
        let mut rb = self.http_client.request(method.clone(), url).headers(headers);

        // Add the body, this is to ensure our GET and DELETE calls succeed.
        if method != Method::GET && method != Method::DELETE {
            rb = rb.json(&body);
        }

        // Build the request.
        Ok(rb.build()?)
    }

    pub async fn get_cfps(
        &self,
    ) -> Result<Vec<Conf>> {
        let request = self.request(
            Method::GET,
            "cfps".to_string(),
            (),
        )?;

        let resp = self.http_client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {:?}", s, resp.text().await?);
            }
        };

        let confs: Vec<Conf> = resp.json().await?;

        Ok(confs)
    }

    pub async fn get_cfp(
        &self,
        cfp_id: i32,
    ) -> Result<Conf> {
        let request = self.request(
            Method::GET,
            format!("{}/{}/", "cfps".to_string(), cfp_id.to_string()),
            (),
        )?;

        let resp = self.http_client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {:?}", s, resp.text().await?);
            }
        };

        let conf: Conf = resp.json().await?;

        Ok(conf)
    }

    pub async fn get_confs(
        &self,
    ) -> Result<Vec<Conf>> {
        let request = self.request(
            Method::GET,
            "conferences".to_string(),
            (),
        )?;

        let resp = self.http_client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {:?}", s, resp.text().await?);
            }
        };

        let confs: Vec<Conf> = resp.json().await?;

        Ok(confs)
    }

    pub async fn get_conf(
        &self,
        conf_id: i32,
    ) -> Result<Conf> {
        let request = self.request(
            Method::GET,
            format!("{}/{}/", "conferences".to_string(), conf_id.to_string()),
            (),
        )?;

        let resp = self.http_client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {:?}", s, resp.text().await?);
            }
        };

        let conf: Conf = resp.json().await?;

        Ok(conf)
    }

    pub async fn get_upcoming(
        &self,
    ) -> Result<Vec<Conf>> {
        let request = self.request(
            Method::GET,
            "upcoming".to_string(),
            ()
        )?;

        let resp = self.http_client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {:?}", s, resp.text().await?);
            }
        };

        let upcoming: Vec<Conf> = resp.json().await?;

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
