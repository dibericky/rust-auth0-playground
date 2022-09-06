use crate::authentication::Auth0;

use super::handlers;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/auth")
        .service(handlers::login)
        .service(handlers::logout)
        .service(handlers::callback)
}
