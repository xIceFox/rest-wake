use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, PaginatorTrait, Set};
use serde::Deserialize;
use crate::models::device;
use crate::models::prelude::{Device};

use crate::State;

#[derive(Deserialize)]
pub struct PageInfo {
    page: Option<u64>,
}

#[get("/{device_name}")]
pub async fn get_device(db: web::Data<State>, path: web::Path<String>) -> impl Responder {
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

    HttpResponse::Ok().body(
        match serde_json::to_string(&device) {
            Ok(value) => value,
            Err(_) => return HttpResponse::InternalServerError().body("Error on serialization!")
        }
    )
}

#[get("")]
pub async fn get_devices(db: web::Data<State>, info: web::Query<PageInfo>) -> impl Responder {
    let page = info.page.unwrap_or(0);
    let paginator = Device::find()
        .paginate(&db.db_conn, 50);

    let devices = match paginator.fetch_page(page).await{
        Ok(value) => value,
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error!")
    };

    match serde_json::to_string(&devices){
        Ok(value) => HttpResponse::Ok().body(value),
        Err(_) => HttpResponse::InternalServerError().body("Deserialization error!")
    }
}

#[post("")]
pub async fn create_device(db: web::Data<State>, mut device: web::Json<device::Model>) -> impl Responder {
    device.0.mac = device.0.mac.to_uppercase();
    return match device.0.into_active_model().insert(&db.db_conn).await {
        Ok(_) => HttpResponse::Ok().body("Inserted device!"),
        Err(_) => return HttpResponse::InternalServerError().body("Insert failed!")
    };
}

#[put("")]
pub async fn update_device(db: web::Data<State>, mut updated_device: web::Json<device::Model>) -> impl Responder {
    updated_device.mac = updated_device.mac.to_uppercase();
    let name = updated_device.name.clone();

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

    let mac = device.mac.clone();

    let mut active_device_model = device.into_active_model();

    let mut changed = false;

    if mac != updated_device.mac {
        active_device_model.mac = Set(updated_device.mac.clone());
        changed = true;
    }

    if changed {
        return match active_device_model.update(&db.db_conn).await {
            Ok(_) => HttpResponse::Ok().body("Updated device!"),
            Err(_) => HttpResponse::InternalServerError().body("There was an error updating the device!")
        };
    };

    HttpResponse::BadRequest().body("Nothing changed on the device!")
}

#[delete("/{device_name}")]
pub async fn delete_device(db: web::Data<State>, path: web::Path<String>) -> impl Responder {
    return match Device::delete_by_id(path.into_inner()).exec(&db.db_conn).await {
        Ok(_) => HttpResponse::Ok().body("Deleted device!"),
        Err(_) => return HttpResponse::InternalServerError().body("Deletion failed!")
    };
}