use super::config::{self, CONFIG};
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use anyhow::{Context, Result};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslVerifyMode};
use tokio::sync::broadcast;

mod handler;
mod services;
mod ws;

pub async fn run() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let (tx, _) = broadcast::channel::<web::Bytes>(128);

    let mut server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(tx.clone()))
            .service(web::resource("/ws").route(web::get().to(ws::echo_heartbeat_ws)))
            .service(web::resource("/ws-broadcast").route(web::get().to(ws::broadcast_ws)))
            .service(web::resource("/send").route(web::post().to(ws::send_to_broadcast_ws)))
            .service(web::scope("api").service(services::telemetry::routes::services()));
        app
    });

    server = match CONFIG.ssl.is_valid {
        true => {
            tracing::warn!("HTTPS");
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder.set_private_key_file(&CONFIG.ssl.key.clone().unwrap(), SslFiletype::PEM)?;
            builder.set_certificate_chain_file(&CONFIG.ssl.crt.clone().unwrap())?;
            builder.set_verify(SslVerifyMode::NONE);
            server.bind_openssl(CONFIG.app.server_url(), builder)
        }
        false => {
            tracing::warn!("HTTP");
            server.bind(CONFIG.app.server_url())
        }
    }
    .context("could not bind server")?;

    Ok(server.run().await?)
}
