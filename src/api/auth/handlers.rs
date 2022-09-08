use crate::authentication::{Auth0, AuthCodeUrl, Authentication};
use actix_web::{cookie::Cookie, get, http::header, web, HttpResponse, Responder};
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
    auth: web::Data<Auth0>,
    query: web::Query<CallbackRequest>,
) -> impl Responder {
    let code = &query.code;

    let token = auth
        .exchange(code)
        .await
        // TODO: error handling
        .unwrap();

    // verify that token is signed by the expected issuer (auth0? )

    let access_token = create_cookie("atk", &token.access_token);
    let id_token = create_cookie("itk", &token.id_token);

    HttpResponse::Found()
        .cookie(access_token)
        .cookie(id_token)
        .append_header(("Location", "http://localhost:6060/api/messages/public"))
        .finish()
}
