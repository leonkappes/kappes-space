mod models;
mod schema;
mod database;
mod routes;
mod auth;
use std::error::Error;

use actix_web::{HttpServer, App, web::{Data, self}, middleware};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::pg::Pg;

use crate::{database::postgres::establish_connection, routes::{index::*, posts::*, auth::login}};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");


fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    println!("Running migrations");
    let migration = run_migrations(&mut pool.get().unwrap());
    if let Ok(_) = migration {
        println!("Migrations successfull")
    }else if let Err(e) = migration {
        println!("Error running migrations: {:?}", e)
    }


    println!("Booting up Rest-Server");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/posts", web::get().to(get_posts))
            .route("/posts/test", web::get().to(create_test_data))
            .route("/posts/author/{uname}", web::get().to(get_by_user))
            .route("/posts", web::post().to(insert_post))
            .route("/auth/login", web::post().to(login))
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
