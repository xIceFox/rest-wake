use std::path::Path;

use actix_web::{App, HttpServer, middleware::Logger, middleware::NormalizePath, web};
use sea_orm::{Database, DatabaseConnection, DbErr};

use migration::{Migrator, MigratorTrait};

mod routes;
mod network;
mod models;

pub struct State {
    db_conn: DatabaseConnection,
}

const IP_ADDR: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let conn = match connect_db().await {
        Ok(value) => value,
        Err(err) => panic!("Error on database connection: {}", err)
    };

    log::info!("Starting HTTP server at http://127.0.0.1:8080");

    HttpServer
    ::new(move || {
        App::new()
            .app_data(web::Data::new(State { db_conn: conn.clone() }))
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(web::scope("/api")
                .service(web::scope("/wake")
                    .service(routes::wake::wake_single)
                    .service(routes::wake::wake_multiple)
                )
                .service(web::scope("/device")
                    .service(routes::device::get_device)
                    .service(routes::device::post_device)
                )
            )
    })
        .bind((IP_ADDR, PORT))?.run().await
}

async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let db_url = "sqlite:db/db.sqlite";

    if !Path::new("db").exists() {
        std::fs::create_dir("db").expect("Could not create db folder!");
    }

    let conn = Database::connect(&format!("{}?mode=rwc", db_url)).await?;
    Migrator::up(&conn, None).await.unwrap();

    Ok(conn)
}