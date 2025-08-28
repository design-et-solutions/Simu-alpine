use crate::server::handler;
use actix_files::NamedFile;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware, rt, web,
};
use tokio::sync::broadcast;

/// Handshake and start WebSocket handler with heartbeats.
pub async fn echo_heartbeat_ws(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler::echo_heartbeat_ws(session, msg_stream));

    Ok(res)
}

/// Send message to clients connected to broadcast WebSocket.
pub async fn send_to_broadcast_ws(
    body: web::Bytes,
    tx: web::Data<broadcast::Sender<web::Bytes>>,
) -> Result<impl Responder, Error> {
    tx.send(body)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent())
}

/// Handshake and start broadcast WebSocket handler with heartbeats.
pub async fn broadcast_ws(
    req: HttpRequest,
    stream: web::Payload,
    tx: web::Data<broadcast::Sender<web::Bytes>>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler::broadcast_ws(session, msg_stream, tx.subscribe()));

    Ok(res)
}
