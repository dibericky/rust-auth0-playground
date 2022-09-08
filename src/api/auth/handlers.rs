use crate::authentication::{Auth0, AuthCodeUrl, Authentication};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/login")]
pub async fn login(auth: web::Data<Auth0>) -> impl Responder {
    let AuthCodeUrl(auth_url) = auth.get_auth_url();

    HttpResponse::Found().header("Location", auth_url).finish()
}

#[get("/logout")]
pub async fn logout() -> impl Responder {
    "logout".to_string()
}

#[derive(Deserialize)]
struct CallbackRequest {
    code: String,
}

#[get("/callback")]
pub async fn callback(
    auth: web::Data<Auth0>,
    query: web::Query<CallbackRequest>,
) -> impl Responder {
    let code = &query.code;

    let token = auth.exchange(code).await;

    // verify that token is signed by the expected issuer (auth0? )

    "callback".to_string()
}
