use actix_web::{get, HttpResponse, post, Responder, web};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use crate::models::device;
use crate::models::prelude::{Device};

use crate::State;


#[get("/{device_name}")]
pub async fn get_device(db: web::Data<State>, path: web::Path<String>) -> impl Responder {
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
    HttpResponse::Ok().body(
        match serde_json::to_string(&device){
            Ok(value) => value,
            Err(_) => return HttpResponse::InternalServerError().body("Error on serialization!")
        }
    )
}

#[post("")]
pub async fn post_device(db: web::Data<State>, device: web::Json<device::Model>) -> impl Responder {
    return match device.0.into_active_model().insert(&db.db_conn).await{
        Ok(_) => HttpResponse::Ok().body("Inserted device!"),
        Err(_) => return HttpResponse::InternalServerError().body("Insert failed!")
    };
}