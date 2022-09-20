mod model;
mod schema;
mod database;
mod routes;
use actix_web::{HttpServer, App, web::{Data, self}, middleware};

use crate::{database::postgres::establish_connection, routes::index::{health, index}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    println!("Booting up Rest-Server");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
