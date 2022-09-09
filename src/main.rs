mod api;
mod authentication;
mod extractors;
mod middlewares;
mod types;
mod cache;

use std::sync::Arc;

use actix_web::{get, web, Responder};
use actix_web::{App, HttpServer};
use authentication::Auth0;
use authentication::AuthCodeUrl;
use cache::{RedisCache, Cache};
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
    let cache = RedisCache::new(redis_config).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
          //  .app_data(web::Data::new(cache))
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
