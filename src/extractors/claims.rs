use crate::types::ErrorMessage;
use actix_web::{
    error::ResponseError,
    http::{StatusCode, Uri},
    Error, FromRequest, HttpResponse,
};
use actix_web_httpauth::{
    extractors::bearer::BearerAuth, headers::www_authenticate::bearer::Bearer,
};
use derive_more::Display;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::Deserialize;
use reqwest::Client;
use std::{collections::HashSet, future::Future, pin::Pin};

#[derive(Clone, Deserialize)]
pub struct Auth0Config {
    pub audience: String,
    pub domain: String,
    pub client_id: String,
    pub connection: String,
    pub redirect_url: String
}

impl Default for Auth0Config {
    fn default() -> Self {
        envy::prefixed("AUTH0_")
            .from_env()
            .expect("Provide missing environment variables for Auth0Client")
    }
}

#[derive(Clone, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u32,
    pub password: String,
    pub username: String
} 

impl Default for RedisConfig {
    fn default() -> Self {
        envy::prefixed("REDIS_")
            .from_env()
            .expect("Provide missing environment variables for Redis client")
    }
}

impl RedisConfig {
    pub fn url(self: &Self) -> String {
        // format!("{}:{}", self.host, self.port)
        format!("redis://{}:{}@{}:{}", self.username, self.password, self.host, self.port)
    }
}

#[derive(Debug, Display)]
enum ClientError {
    #[display(fmt = "authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
    #[display(fmt = "decode")]
    Decode(jsonwebtoken::errors::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
}

impl ResponseError for ClientError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(
                    "Authorization header value must follow this format: Bearer access-token"
                        .to_string(),
                ),
                message: "Bad credentials".to_string(),
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(msg.to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::UnsupportedAlgortithm(alg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(format!(
                    "Unsupported encryption algortithm expected RSA got {:?}",
                    alg
                )),
                message: "Bad credentials".to_string(),
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    _permissions: Option<HashSet<String>>,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let config = req.app_data::<Auth0Config>().unwrap().clone();
        let extractor = BearerAuth::extract(req);
        Box::pin(async move {
            let credentials = extractor.await.map_err(ClientError::Authentication)?;
            let token = credentials.token();
            let header = decode_header(token).map_err(ClientError::Decode)?;
            let kid = header.kid.ok_or_else(|| {
                ClientError::NotFound("kid not found in token header".to_string())
            })?;
            let domain = config.domain.as_str();
            let jwks: JwkSet = Client::new()
                .get(
                    format!("https://{domain}/.well-known/jwks.json")
                )
                .send()
                .await
                .map_err(|_| "unable to get jwks")
                // TODO: error handler
                .unwrap()
                .json()
                .await
                // TODO: error handler
                .unwrap();
            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| ClientError::NotFound("No JWK found for kid".to_string()))?;
            match jwk.clone().algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.set_audience(&[config.audience]);
                    validation.set_issuer(&[Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/")
                        .build()
                        .unwrap()]);
                    let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(ClientError::Decode)?;
                    let token =
                        decode::<Claims>(token, &key, &validation).map_err(ClientError::Decode)?;
                    Ok(token.claims)
                }
                algorithm => Err(ClientError::UnsupportedAlgortithm(algorithm).into()),
            }
        })
    }
}
