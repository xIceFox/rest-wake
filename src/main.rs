use std::path::Path;
use actix_web::{web, App, HttpServer, middleware::Logger, middleware::NormalizePath};
use sqlx::{Error, Pool, Sqlite};
use sqlx::sqlite::{SqlitePoolOptions};

mod routes;
mod network;
mod models;

pub struct DB {
    pool: Pool<Sqlite>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let db_pool = match connect_db().await {
        Ok(value) => value,
        Err(err) => panic!("Error on database creation: {}", err)
    };

    log::info!("Starting HTTP server at http://127.0.0.1:8080");

    HttpServer
    ::new(move || {
        App::new()
            .app_data(web::Data::new(DB { pool: db_pool.clone() }))
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(web::scope("/api")
                .service(web::scope("/wake")
                    .service(routes::wake::wake_single)
                    .service(routes::wake::wake_multiple)
                )
                .service(web::scope("/device")
                    .service(routes::device::get_device)
                )
            )
    })
        .bind(("127.0.0.1", 8080))?.run().await
}

async fn connect_db() -> Result<Pool<Sqlite>, Error> {
    let db_url = "db/db.sqlite";

    if !Path::new("db").exists() {
        std::fs::create_dir("db").expect("Could not create db folder!");
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("{}?mode=rwc", db_url)).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}