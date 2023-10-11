use actix_web::{post, Responder, HttpResponse, web};
use crate::network::magic_packet;
use crate::network::mac_address;
use crate::network::mac_address::MacAddress6;

#[post("/{mac_addr}")]
pub async fn wake_single(path: web::Path<String>) -> impl Responder {
    let mac = match mac_address::MacAddress6::try_from(path.into_inner()) {
        Ok(value) => value,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string())
    };
    return match magic_packet::send_wol_packet(mac) {
        Ok(_) => HttpResponse::Ok().body("Magic packet successfully sent!"),
        Err(err) => return HttpResponse::InternalServerError().body(err)
    };
}

#[post("")]
pub async fn wake_multiple(mac_addresses: web::Json<Vec<String>>) -> impl Responder {
    let mac_addresses_parsed: Vec<MacAddress6> = match mac_address::parse_multiple_strings(mac_addresses.0) {
        Ok(value) => value,
        Err(err) => return HttpResponse::BadRequest().body(err)
    };
    return match magic_packet::send_wol_packets(mac_addresses_parsed) {
        Ok(_) => HttpResponse::Ok().body("Magic packets successfully sent!"),
        Err(err) => return HttpResponse::InternalServerError().body(err)
    };
}



