use actix_web::{get, Responder, HttpResponse};

#[get("")]
pub async fn info_message() -> impl Responder {
    return HttpResponse::Ok().body("Message");
}


