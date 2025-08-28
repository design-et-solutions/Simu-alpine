use crate::server::services::{telemetry::handlers};
use actix_web::{
    HttpResponse, Scope,
    web::{self},
};

pub fn services() -> Scope {
    web::scope("/telemetry")
        .route("", web::post().to(handlers::add_speed))
}
