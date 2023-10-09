use actix_web::{web, App, HttpServer, middleware::Logger, middleware::NormalizePath};

mod routes;
mod network;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    network::magic_packet::send_wol([0x7C, 0x10, 0xC9, 0x3C, 0x68, 0x1E]).expect("TODO: panic message");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(web::scope("/api")
                .service(web::scope("/wake")
                    .service(routes::wake::info_message)
                )
            )
    })
        .bind(("127.0.0.1", 8080))?.run().await
}