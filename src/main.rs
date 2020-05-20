mod config;
mod models;
mod db;
mod handlers;

use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Releasing a velociraptor in {}:{}...", config.server.host, config.server.port);

    HttpServer::new(move || {

        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/url", web::get().to(create_redirect))
   })
   .bind(format!("{}:{}", config.server.host, config.server.port))?
   .run()
   .await
}
