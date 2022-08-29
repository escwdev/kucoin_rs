use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("Serde issue parsing error {}", _0)]
    Serde(#[from] serde_json::Error),
    #[error("Websocket error {}", _0)]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("REST Call error {}", _0)]
    HTTP(#[from] reqwest::Error),
    #[error("Other issue {}", _0)]
    Other(String),
}
