#[derive(Fail, Debug)]
pub enum APIError {
    #[fail(display = "Serde issue parsing error {}", _0)]
    Serde(#[fail(cause)] serde_json::Error),
    #[fail(display = "Websocket error {}", _0)]
    Websocket(#[fail(cause)] tokio_tungstenite::tungstenite::Error),
    #[fail(display = "REST Call error {}", _0)]
    HTTP(#[fail(cause)] reqwest::Error),
    #[fail(display = "Other issue {}", _0)]
    Other(String),
}

impl APIError {

}

impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> Self {
        APIError::HTTP(err)
    }
}

impl From<serde_json::Error> for APIError {
    fn from(err: serde_json::Error) -> Self {
        APIError::Serde(err)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for APIError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        APIError::Websocket(err)
    }
}