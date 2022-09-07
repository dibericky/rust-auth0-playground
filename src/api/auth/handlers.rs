use crate::{
    authentication::{Auth0, AuthCodeUrl, Authentication},
    extractors::Auth0Config,
};
use actix_web::{get, web, Responder, HttpResponse};

#[get("/login")]
pub async fn login(auth: web::Data<Auth0>) -> impl Responder {
    let AuthCodeUrl(auth_url) = auth.get_auth_url();

    HttpResponse::Found().header("Location", auth_url).finish()
}

#[get("/logout")]
pub async fn logout() -> impl Responder {
    "logout".to_string()
}

#[get("/callback")]
pub async fn callback() -> impl Responder {
    "callback".to_string()
}
