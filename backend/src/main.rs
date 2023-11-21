use std::collections::HashMap;
use std::env;
use std::fs::create_dir_all;
use std::path::Path;

use actix_web::{App, HttpServer, middleware::Logger, middleware::NormalizePath, web};
use dotenv::dotenv;
use log::LevelFilter;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, RuntimeErr};

mod routes;
mod network;
mod models;

pub struct State {
    db_conn: DatabaseConnection,
}

async fn connect_db(db_url: &String) -> Result<DatabaseConnection, DbErr> {
    let db_url_parts = db_url.split(':').collect::<Vec<&str>>();
    if db_url_parts.len() != 2 || db_url_parts[0] != "sqlite" {
        return Err(DbErr::Conn(RuntimeErr::Internal(String::from("Wrong database url specified, should be like: 'sqlite:[filesystem_path]'"))));
    }

    let folder_path = Path::new(db_url_parts[1]).parent().unwrap();
    if !folder_path.exists() {
        create_dir_all(folder_path).expect("Folders to db could not be created! Try to create the folder path yourself!");
    }

    let mut options = ConnectOptions::new(format!("{}?mode=rwc", db_url));
    options.sqlx_logging_level(LevelFilter::Debug);

    let conn = Database::connect(options).await?;

    Migrator::up(&conn, None).await.unwrap();

    Ok(conn)
}

async fn get_settings() -> HashMap<String, String> {
    dotenv().ok();

    let default_settings = [
        ("DATABASE_URL", "sqlite:db/db.sqlite"),
        ("IP", "localhost"),
        ("PORT", "8080"),
        ("RUST_LOG", "INFO")
    ];

    let mut settings: HashMap<String, String> = HashMap::new();

    for (key, value) in default_settings {
        match env::var(key) {
            Ok(env_var) => {
                settings.insert(String::from(key), env_var);
            }
            Err(_) => {
                println!("{} not found in environment variables. Defaulting to: \"{}\"", key, value);
                settings.insert(String::from(key), String::from(value));
                env::set_var(key, value);
            }
        };
    }

    if cfg!(debug_assertions) {
        println!("Debug binary detected, setting LogLevel to DEBUG!");
        env::set_var("RUST_LOG", "DEBUG");
    }

    settings
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_settings().await;

    let db_url = settings.get("DATABASE_URL").expect("Database URL is not set!");
    let ip = settings.get("IP").expect("IP is not set!");
    let port = settings.get("PORT")
        .expect("Port is not set!")
        .parse::<u16>()
        .expect("Port could not be parsed!");

    env_logger::builder()
        .init();

    let conn = match connect_db(db_url).await {
        Ok(value) => value,
        Err(err) => panic!("Error on database connection: {}", err)
    };

    log::info!("Starting HTTP server at http://{ip}:{port}");

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
                    .service(routes::wake::wake_with_name)
                    .service(routes::wake::wake_multiple_with_name)
                )
                .service(web::scope("/device")
                    .service(routes::device::get_device)
                    .service(routes::device::create_device)
                    .service(routes::device::update_device)
                    .service(routes::device::delete_device)
                )
                .service(web::scope("/devices")
                    .service(routes::devices::get_devices)
                )
            )
    })
        .bind((String::from(ip), port))?.run().await
}