use crate::authentication::{Auth0, AuthCodeUrl, Authentication};
use actix_session::Session;
use actix_web::{cookie::Cookie, get, http::header, web, HttpResponse, Responder, Result};
use reqwest::StatusCode;
use serde::Deserialize;

#[get("/login")]
pub async fn login(auth: web::Data<Auth0>) -> impl Responder {
    let AuthCodeUrl(auth_url) = auth.get_auth_url();

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[get("/logout")]
pub async fn logout() -> impl Responder {
    "logout".to_string()
}

#[derive(Deserialize)]
pub struct CallbackRequest {
    code: String,
}

fn create_cookie<'a>(key: &'a str, value: &'a str) -> Cookie<'a> {
    Cookie::build(key, value)
        .domain("localhost")
        .path("/")
        .finish()
}

#[get("/callback")]
pub async fn callback(
    session: Session,
    auth: web::Data<Auth0>,
    query: web::Query<CallbackRequest>,
) -> Result<HttpResponse> {
    let code = &query.code;

    let token = auth
        .exchange(code)
        .await.map_err(crate::types::create_err)?;

    // verify that token is signed by the expected issuer (auth0? )

    session.insert("access_token", token.access_token.clone())?;
    session.insert("id_token", token.id_token.clone())?;
    session.renew();

    Ok(HttpResponse::Found()
        .append_header(("Location", "http://localhost:6060/api/messages/public"))
        .finish())
}

