use actix_web::{get, post, Responder, HttpResponse, web};
use crate::network::magic_packet;

#[get("")]
pub async fn info_message() -> impl Responder {
    return HttpResponse::Ok().body("Test Endpoint");
}

#[post("/{mac_addr}")]
pub async fn wake_with_max(path : web::Path<String>) -> impl Responder {
    let mac = path.into_inner();
    //TODO implement string to MAC in own mac_address class
    magic_packet::send_wol([0x11, 0x11, 0x11, 0x11, 0x11, 0x11]).expect("TODO: panic message");
    return HttpResponse::NotImplemented();
}



