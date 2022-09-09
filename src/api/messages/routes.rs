use super::handlers;
use actix_web::{web::{self, resource}, Scope};

pub fn routes() -> Scope {
    web::scope("/messages")
        .service(handlers::admin)
        .service(handlers::protected)
        .service(handlers::public)
}
