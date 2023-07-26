#![allow(dead_code, unused_imports, unused_variables)]
mod controller;
mod model;
mod service;
mod utils;

use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    error::InternalError,
    middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use controller::user_controller::login_user;
use controller::{main_controller::get_index, signup_controller::signup};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();
    let redis_connection_string = "127.0.0.1:6379";
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // cookie session middleware
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(redis_connection_string),
                    secret_key.clone(),
                )
                // allow the cookie to be accessed from javascript
                .cookie_http_only(false)
                // allow the cookie only from the current domain
                .cookie_same_site(SameSite::Strict)
                .build(),
            )
            .service(get_index)
            .service(login_user)
            .service(signup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
