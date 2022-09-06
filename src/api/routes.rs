use crate::authentication::Auth0;
use crate::authentication::Authentication;

use super::auth;
use super::messages;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/api")
        .service(messages::routes())
        .service(auth::routes())
}
