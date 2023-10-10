use actix_web::{web, App, HttpServer, middleware::Logger, middleware::NormalizePath};

mod routes;
mod network;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(web::scope("/api")
                .service(web::scope("/wake")
                    .service(routes::wake::info_message)
                    .service(routes::wake::wake_with_max)
                )
            )
    })
        .bind(("127.0.0.1", 8080))?.run().await
}