use crate::models::telemetry::NewSpeed;
use actix_web::{HttpResponse, Responder, web};

pub async fn add_speed(new_speed: web::Json<NewSpeed>) -> impl Responder {
    tracing::info!("hihi");
    HttpResponse::Ok()
}
