use actix_web::{get, HttpResponse, Responder, web};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde::Deserialize;
use crate::models::prelude::Device;
use crate::State;

#[derive(Deserialize)]
pub struct PageInfo {
    page: Option<u64>,
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