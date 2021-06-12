use reqwest;
use std::collections::HashMap;
use std::time::Duration;

use base64::encode;
use failure;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::json;
use sha2::Sha256;

use super::error::APIError;
use super::model::Method;
use super::utils::get_time;

// Alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct Credentials {
    api_key: String,
    secret_key: String,
    passphrase: String,
}

impl Credentials {
    pub fn new<S: Into<String>>(api_key: S, secret_key: S, passphrase: S) -> Self {
        Credentials {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            passphrase: passphrase.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum KucoinEnv {
    Live,
    Sandbox,
}

#[derive(Debug, Clone)]
pub struct Kucoin {
    credentials: Option<Credentials>,
    environment: KucoinEnv,
    pub prefix: String,
    pub client: reqwest::Client,
}

impl Kucoin {
    pub fn new(
        environment: KucoinEnv,
        credentials: Option<Credentials>,
    ) -> Result<Self, failure::Error> {
        let client = reqwest::Client::builder()
            // .use_rustls_tls()
            .timeout(Duration::from_secs(60))
            .build()?;
        let prefix = match environment {
            KucoinEnv::Live => String::from("https://api.kucoin.com"),
            KucoinEnv::Sandbox => String::from("https://openapi-sandbox.kucoin.com"),
        };
        Ok(Kucoin {
            credentials,
            environment,
            prefix,
            client,
        })
    }

    // Generic get request for internal library use.
    // Matches credentials for signed vs. unsigned API calls
    pub async fn get<S: Into<String>>(
        &self,
        url: S,
        sign: Option<HeaderMap>,
    ) -> Result<reqwest::Response, APIError> {
        let req_url = reqwest::Url::parse(&url.into()).unwrap();
        match sign {
            Some(sign) => {
                let resp = self.client.get(req_url).headers(sign).send().await?;
                if resp.status().is_success() {
                    Ok(resp)
                } else {
                    Ok(resp)
                }
            }
            None => {
                let resp = self.client.get(req_url).send().await?;
                if resp.status().is_success() {
                    Ok(resp)
                } else {
                    Ok(resp)
                }
            }
        }
    }

    pub async fn post(
        &self,
        url: String,
        sign: Option<HeaderMap>,
        params: Option<HashMap<String, String>>,
    ) -> Result<reqwest::Response, APIError> {
        let req_url = reqwest::Url::parse(&url).unwrap();
        if let Some(s) = sign {
            if let Some(p) = params {
                let resp = self
                    .client
                    .post(req_url)
                    .headers(s)
                    .json(&json!(p))
                    .send()
                    .await?;
                if resp.status().is_success() {
                    Ok(resp)
                } else {
                    Ok(resp)
                }
            } else {
                let resp = self.client.post(req_url).headers(s).send().await?;
                if resp.status().is_success() {
                    Ok(resp)
                } else if resp.status().is_server_error() {
                    Ok(resp)
                } else {
                    Ok(resp)
                }
            }
        } else {
            panic!("Unsigned POST request...")
        }
    }

    pub async fn delete(
        &self,
        url: String,
        sign: Option<HeaderMap>,
    ) -> Result<reqwest::Response, APIError> {
        let req_url = reqwest::Url::parse(&url).unwrap();
        if let Some(s) = sign {
            let resp = self.client.delete(req_url).headers(s).send().await?;
            if resp.status().is_success() {
                Ok(resp)
            } else if resp.status().is_server_error() {
                Ok(resp)
            } else {
                Ok(resp)
            }
        } else {
            panic!("Unsigned DELETE request...")
        }
    }

    pub fn sign_headers(
        &self,
        endpoint: String,
        params: Option<&HashMap<String, String>>,
        query: Option<String>,
        method: Method,
    ) -> Result<HeaderMap, failure::Error> {
        let mut headers = HeaderMap::new();
        let nonce = get_time().to_string();
        let mut api_key: &str = "";
        let mut secret_key: &str = "";
        let mut passphrase: &str = "";
        let mut str_to_sign: String = String::new();
        match &self.credentials {
            Some(c) => {
                api_key = &c.api_key;
                secret_key = &c.secret_key;
                passphrase = &c.passphrase;
            }
            None => (),
        }
        match method {
            Method::GET => {
                let meth = "GET";
                if let Some(q) = query {
                    // let query = format_query(&p);
                    str_to_sign = format!("{}{}{}{}", nonce, meth, endpoint, q);
                } else {
                    str_to_sign = format!("{}{}{}", nonce, meth, endpoint)
                }
            }
            Method::POST => {
                let meth = "POST";
                if let Some(p) = params {
                    let q = json!(&p);
                    str_to_sign = format!("{}{}{}{}", nonce, meth, endpoint, q);
                } else {
                    str_to_sign = format!("{}{}{}", nonce, meth, endpoint)
                }
            }
            Method::PUT => {}
            Method::DELETE => {
                let meth = "DELETE";
                if let Some(q) = query {
                    // let query = format_query(&p);
                    str_to_sign = format!("{}{}{}{}", nonce, meth, endpoint, q);
                } else {
                    str_to_sign = format!("{}{}{}", nonce, meth, endpoint)
                }
            }
        }
        let mut hmac_sign =
            HmacSha256::new_varkey(secret_key.as_bytes()).expect("HMAC can take key of any size");
        hmac_sign.input(str_to_sign.as_bytes());
        let sign_result = hmac_sign.result();
        let sign_bytes = sign_result.code();
        let sign_digest = encode(&sign_bytes);
        let mut hmac_passphrase =
            HmacSha256::new_varkey(secret_key.as_bytes()).expect("HMAC can take key of any size");
        hmac_passphrase.input(passphrase.as_bytes());
        let passphrase_result = hmac_passphrase.result();
        let passphrase_bytes = passphrase_result.code();
        let passphrase_digest = encode(&passphrase_bytes);
        headers.insert(
            HeaderName::from_static("kc-api-key"),
            HeaderValue::from_str(&api_key).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-sign"),
            HeaderValue::from_str(&sign_digest).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-timestamp"),
            HeaderValue::from_str(&nonce).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-passphrase"),
            HeaderValue::from_str(&passphrase_digest).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("kc-api-key-version"),
            HeaderValue::from_str("2").unwrap(),
        );
        Ok(headers)
    }
}
