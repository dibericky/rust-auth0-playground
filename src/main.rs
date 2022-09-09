mod api;
mod authentication;
mod extractors;
mod middlewares;
mod types;

use std::sync::Arc;

use actix_session::SessionMiddleware;
use actix_session::storage::RedisActorSessionStore;
use actix_web::{get, web, Responder};
use actix_web::{App, HttpServer};
use authentication::Auth0;
use authentication::AuthCodeUrl;
use dotenv::dotenv;

use crate::authentication::Authentication;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = types::Config::default();
    
    let auth0_config = extractors::Auth0Config::default();
    let authentication = Auth0::new(&auth0_config);
    
    let redis_config = extractors::RedisConfig::default();
    let redis_url = redis_config.url();
    println!("redis url: {}", redis_url);    
    let private_key = actix_web::cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
            .wrap(
                SessionMiddleware::builder(
                    //TODO: how to set password?
                    RedisActorSessionStore::new(&redis_url),
                    private_key.clone(),
                )
                .cookie_name("test-session".to_string())
                .build(),
            )
            .app_data(web::Data::new(authentication.clone()))
            .wrap(middlewares::cors(&config.client_origin_url))
            // TODO: how to error handling with current version of actix_web?
            // .wrap(middlewares::err_handlers())
            .wrap(middlewares::security_headers())
            .wrap(middlewares::logger())
            .service(api::routes())
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
