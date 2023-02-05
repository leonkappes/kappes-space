mod auth;
mod config;
mod database;
mod models;
mod routes;
mod schema;
use std::{error::Error, sync::atomic::AtomicBool};

use actix_web::{
    middleware::{self, Compat},
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::pg::Pg;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::{
    auth::bearer_auth_validator,
    config::Config,
    database::postgres::establish_connection,
    routes::{
        auth::{login, signup},
        index::*,
        posts::*,
        settings::toggle_registration,
    },
};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
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
    } else if let Err(e) = migration {
        println!("Error running migrations: {:?}", e)
    }

    let config = web::Data::new(Config {
        allow_signups: AtomicBool::new(true),
    });

    let auth = HttpAuthentication::with_fn(bearer_auth_validator);

    println!("Booting up Rest-Server");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/posts")
                    .route("", web::get().to(get_posts))
                    .route("/test", web::get().to(create_test_data))
                    .route("/author/{uname}", web::get().to(get_by_user))
                    .route("", web::post().to(insert_post)),
            )
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(login))
                    .route("/signup", web::post().to(signup)),
            )
            .service(web::scope("/settings").route("", web::post().to(toggle_registration)))
            .wrap(middleware::Logger::default())
            .wrap(Compat::new(auth.clone()))
            .app_data(Data::new(pool.clone()))
            .app_data(config.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
