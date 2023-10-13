use actix_web::{post, Responder, HttpResponse, web};
use sea_orm::EntityTrait;
use crate::models::device;
use crate::models::prelude::Device;
use crate::network::magic_packet;
use crate::network::mac_address;
use crate::network::mac_address::MacAddress6;
use crate::State;

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

#[post("/device/{device_name}")]
pub async fn wake_with_name(db: web::Data<State>, path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let option: Option<device::Model> = match Device::find_by_id(name).one(&db.db_conn).await {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Internal Server Error!")
        }
    };
    let device  = match option {
        Some(value) => value,
        None => return HttpResponse::NotFound().body("Device not found!")
    };
    let mac = match MacAddress6::try_from(device.mac){
        Ok(value) => value,
        Err(err) => return HttpResponse::InternalServerError().body(err)
    };
    return match magic_packet::send_wol_packet(mac.clone()) {
        Ok(_) => HttpResponse::Ok().body(format!("Magic packet successfully sent to {mac}!")),
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



