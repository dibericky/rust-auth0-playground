use super::types::{Message, Metadata};
use crate::extractors::Claims;
use actix_web::{get, web, Responder};

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
pub async fn public(req: actix_web::HttpRequest) -> impl Responder {
    let cookies = req.cookies().unwrap()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",");
    web::Json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "basic-authorization".to_string(),
        },
        text: cookies, 
    })
}
