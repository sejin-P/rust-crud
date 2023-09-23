pub mod model;
pub mod errors;
pub mod api;

use std::fmt::Display;
use actix_web::{ web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use mysql::{Pool};
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};

use crate::api::user;
use crate::api::post;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    // TODO make it config var(read from env), dependency injection practically
    let url = "mysql://root:password@localhost:3306/abc";
    let pool = Pool::new(url).expect("failed to create pool");
    let shared_data = web::Data::new(pool);

    // to force the closure to take ownership of `shared_data` (and any other referenced variables), use the `move` keyword
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(shared_data.clone())
            .service(user::get_user)
            .service(user::create_user)
            .service(user::update_user)
            .service(user::delete_user)
            .service(post::get_post)
            .service(post::create_post)
            .service(post::update_post)
            .service(post::delete_post)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
