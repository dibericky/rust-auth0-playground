use serde::{Deserialize, Serialize};
use actix_web::ResponseError;
use std::fmt::{Display, Formatter};
#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    pub port: u16,
    pub client_origin_url: String,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("Provide missing environment variables for Config")
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    pub message: String,
}


pub fn create_err(err: anyhow::Error) -> Error {
    Error(err)
}

/// Necessary because of this issue: https://github.com/actix/actix-web/issues/1711
#[derive(Debug)]
pub struct Error(anyhow::Error);

impl ResponseError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl<T> From<T> for Error
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        Error(t.into())
    }
}