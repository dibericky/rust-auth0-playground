use super::types::{Message, Metadata};
use crate::extractors::Claims;
use actix_session::Session;
use actix_web::{get, web, Responder, Result, HttpResponse};

#[get("/admin")]
pub async fn admin(_claims: Claims) -> impl Responder {
    web::Json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "basic-authorization".to_string(),
        },
        text: "This is an admin message.".to_string(),
    })
}

#[get("/protected")]
pub async fn protected(_claims: Claims) -> impl Responder {
    web::Json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "basic-authorization".to_string(),
        },
        text: "This is a protected message.".to_string(),
    })
}

#[get("/public")]
pub async fn public(session: Session, req: actix_web::HttpRequest) ->  Result<HttpResponse> {

    let access_token = match session.get::<String>("access_token")? {
        None => return Ok(HttpResponse::Unauthorized().finish()),
        Some(tk) => tk
    };

    Ok(HttpResponse::Ok().json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "basic-authorization".to_string(),
        },
        text: access_token, 
    }))
}
