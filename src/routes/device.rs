use actix_web::{Responder, HttpResponse, web, get};
use sqlx::{Error, Row};
use crate::DB;

#[get("/{device_name}")]
pub async fn get_device(db: web::Data<DB>, path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let device_row =
        match sqlx::query("SELECT name,mac FROM DEVICES WHERE name = $1")
            .bind(name.clone())
            .fetch_one(&db.pool)
            .await {
            Ok(value) => { value }
            Err(err) => {
                return match err {
                    Error::RowNotFound => HttpResponse::NotFound().body("Device not found!"),
                    e => HttpResponse::InternalServerError().body(e.to_string())
                };
            }
        };
    HttpResponse::Ok().body(
        format!("{}: {}",
                device_row.get::<String, &str>("name"),
                device_row.get::<String, &str>("mac"))
    )
}