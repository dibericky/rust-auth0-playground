use serde::Deserialize;

use crate::extractors::Auth0Config;

#[derive(Clone, Deserialize)]
pub struct Auth0 {
    domain: String,
    client_id: String,
    connection: String,
    redirect_uri: String
}

impl Auth0 {
    pub fn new(auth0_config: &Auth0Config) -> Self {
        Self {
           domain: auth0_config.domain.clone(),
           client_id: auth0_config.client_id.clone(),
           connection: auth0_config.connection.clone(),
           redirect_uri: auth0_config.redirect_url.clone() 
        }
    }
}

pub struct AuthCodeUrl(pub String);

impl AuthCodeUrl {
    pub fn with_state(self: &Self, state: String) -> String {
        format!("{}&state={}", self.0, state)
    }
}

pub trait Authentication {
    fn get_auth_url(&self) -> AuthCodeUrl;
}

impl Authentication for Auth0 {
    fn get_auth_url(&self) -> AuthCodeUrl {
        let domain = self.domain.clone();
        let response_type = "code";
        let client_id = self.client_id.clone();
        let connection = self.connection.clone();
        let redirect_uri = self.redirect_uri.clone();
        let url = format!("https://{domain}/authorize?response_type={response_type}&client_id={client_id}&connection={connection}&redirect_uri={redirect_uri}");
        AuthCodeUrl(url)
    }
}