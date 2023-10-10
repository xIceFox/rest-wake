use actix_web::{get, post, Responder, HttpResponse, web};
use crate::network::magic_packet;
use crate::network::mac_address;

#[get("")]
pub async fn info_message() -> impl Responder {
    return HttpResponse::Ok().body("Test Endpoint");
}

#[post("/{mac_addr}")]
pub async fn wake_with_max(path : web::Path<String>) -> impl Responder {
    let mac = match mac_address::MacAddress6::try_from(path.into_inner()) {
        Ok(value) => value,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string())
    };
    return match magic_packet::send_wol(mac) {
        Ok(_) => HttpResponse::Ok().body("Sent magic packet!"),
        Err(err) => return HttpResponse::InternalServerError().body(err)
    };
}



