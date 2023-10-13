use actix_web::{post, Responder, HttpResponse, web};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, SelectColumns};
use crate::models::device;
use crate::models::prelude::Device;
use crate::network::magic_packet;
use crate::network::mac_address;
use crate::network::mac_address::MacAddress6;
use crate::State;

#[post("/mac/{mac_addr}")]
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

#[post("/mac")]
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

#[post("/device/{device_name}")]
pub async fn wake_with_name(db: web::Data<State>, path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let option: Option<device::Model> = match Device::find_by_id(name).one(&db.db_conn).await {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Internal Server Error!");
        }
    };
    let device = match option {
        Some(value) => value,
        None => return HttpResponse::NotFound().body("Device not found!")
    };
    let mac = match MacAddress6::try_from(device.mac) {
        Ok(value) => value,
        Err(err) => return HttpResponse::InternalServerError().body(err)
    };
    return match magic_packet::send_wol_packet(mac.clone()) {
        Ok(_) => HttpResponse::Ok().body(format!("Magic packet successfully sent to {mac}!")),
        Err(err) => return HttpResponse::InternalServerError().body(err)
    };
}

#[post("/devices")]
pub async fn wake_multiple_with_name(db: web::Data<State>, device_names: web::Json<Vec<String>>) -> impl Responder {
    let device_count = device_names.0.len();

    let devices = match device::Entity::find()
        .select_column(device::Column::Mac)
        .filter(device::Column::Name.is_in(device_names.0))
        .all(&db.db_conn).await {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Internal Server Error!");
        }
    };

    if devices.len() == 0 {
        return HttpResponse::NotFound().body("None of the devices found!");
    }

    let not_all_found = devices.len() != device_count;

    let mut mac_addresses: Vec<MacAddress6> = Vec::new();

    for device in devices {
        mac_addresses.push(match MacAddress6::try_from(device.mac) {
            Ok(value) => value,
            Err(err) => return HttpResponse::InternalServerError().body(format!("There was an error parsing a device: {}", err))
        })
    }

    return match magic_packet::send_wol_packets(mac_addresses) {
        Ok(_) => {
            if not_all_found {
                return HttpResponse::PartialContent().body("Not all devices were found! Sent magic packets to all found devices!");
            }
            HttpResponse::Ok().body(format!("Magic packets successfully sent to devices!"))
        }
        Err(err) => return HttpResponse::InternalServerError().body(err)
    };
}



