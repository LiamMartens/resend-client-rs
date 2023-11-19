use const_format::concatcp;
use reqwest::{header::HeaderValue, RequestBuilder};
use std::collections::HashMap;

const VERSION: &'static str = "0.1.0";
const DEFAULT_BASE_URL: &'static str = "https://api.resend.com";
const USER_AGENT: &'static str = concatcp!("resend-rust/", VERSION);
const CONTENT_TYPE: &'static str = "application/json";

pub struct ReqClient {
    pub client: reqwest::Client,
    pub api_key: String,
    pub base_url: url::Url,
    pub user_agent: String,
    pub headers: HashMap<String, String>,
}

impl ReqClient {
    pub fn new(api_key: String) -> Self {
        ReqClient::new_custom(reqwest::Client::new(), api_key)
    }

    pub fn new_custom(http_client: reqwest::Client, api_key: String) -> Self {
        let client = ReqClient {
            client: http_client,
            api_key: api_key,
            base_url: url::Url::parse(DEFAULT_BASE_URL).unwrap(),
            user_agent: USER_AGENT.to_string(),
            headers: HashMap::new(),
        };
        client
    }

    pub fn new_request(&self, method: reqwest::Method, path: &str) -> RequestBuilder {
        let mut url = self.base_url.clone();
        url.set_path(&path[..]);
        let mut req = self.client.request(method, url);
        // update request headers
        for (key, value) in self.headers.iter() {
            req = req.header(key, value);
        }
        req = req.header(
            reqwest::header::ACCEPT,
            HeaderValue::from_static(CONTENT_TYPE),
        );
        req = req.header(
            reqwest::header::USER_AGENT,
            HeaderValue::from_bytes(&self.user_agent.as_bytes()).unwrap(),
        );
        req = req.header(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {}", self.api_key).as_bytes()).unwrap(),
        );

        req
    }

    pub fn new_body_request<T: serde::Serialize + ?Sized>(
        &self,
        method: reqwest::Method,
        path: &str,
        json: Option<&T>,
    ) -> RequestBuilder {
        let mut req = self.new_request(method, path);
        // update request body
        if json.is_some() {
            req = req.header(reqwest::header::CONTENT_TYPE, CONTENT_TYPE);
            req = req.body(serde_json::to_string(&json.unwrap()).unwrap());
        }
        req
    }
}
