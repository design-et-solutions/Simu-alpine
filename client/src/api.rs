use crate::TelemetryCtx;
use dioxus::prelude::*;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use gloo_timers::future::IntervalStream;
use reqwest::Client;
use reqwest_websocket::{Bytes, Error};
use reqwest_websocket::{Message, RequestBuilderExt};
use serde::Serialize;
use std::mem;
use std::slice;
use wasm_bindgen_futures::spawn_local;

// Vec3 equivalent
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AcsVehicleInfo {
    car_id: i32,
    driver_name: [u8; 64],
    pub car_model: [u8; 64],
    pub speed_ms: f32,
    pub best_lap_ms: i32,
    pub lap_count: i32,
    current_lap_invalid: i32,
    pub current_lap_time_ms: i32,
    last_lap_time_ms: i32,
    world_position: Vec3,
    is_car_in_pitline: i32,
    is_car_in_pit: i32,
    car_leaderboard_position: i32,
    car_realtime_leaderboard_position: i32,
    pub sp_line_length: f32,
    is_connected: i32,
    pub suspension_damage: [f32; 4],
    pub engine_life_left: f32,
    tyre_inflation: [f32; 4],
}

// Safety: only valid if the bytes really are an AcsVehicleInfo
fn parse_vehicle_info(bytes: &[u8]) -> Option<AcsVehicleInfo> {
    if bytes.len() < mem::size_of::<AcsVehicleInfo>() {
        return None;
    }
    let ptr = bytes.as_ptr() as *const AcsVehicleInfo;
    unsafe { Some(*ptr) }
}

pub const BASE_URL: &str = "ws://127.0.0.1:3000/ws-broadcast";

pub async fn get_ws() -> Result<(), Error> {
    let response = Client::default().get(BASE_URL).upgrade().send().await?;
    let mut websocket = response.into_websocket().await?;
    let (mut sink, mut stream) = websocket.split();
    spawn_local(async move {
        let mut interval = IntervalStream::new(5_000); // every 5000 ms (5s)
        while interval.next().await.is_some() {
            if sink.send(Message::Ping(Bytes::new())).await.is_err() {
                break; // connection closed
            }
        }
    });
    while let Some(message) = stream.try_next().await? {
        match message {
            Message::Binary(bytes) => {
                if let Some(car) = parse_vehicle_info(&bytes) {
                    let TelemetryCtx(mut telemetry_ctx) = use_context::<TelemetryCtx>();
                    telemetry_ctx.set(Some(car));
                    // // Copy fields safely into locals
                    // let car_id = car.car_id;
                    // let speed_ms = car.speed_ms;

                    // // Convert driver_name bytes -> String
                    // let name_bytes: Vec<u8> = car
                    //     .driver_name
                    //     .iter()
                    //     .copied()
                    //     .take_while(|&c| c != 0)
                    //     .collect();
                    // let driver_name = String::from_utf8_lossy(&name_bytes);

                    // tracing::info!(
                    //     "Car {} | Speed: {:.2} m/s | Driver: {}",
                    //     car_id,
                    //     speed_ms,
                    //     driver_name
                    // );
                } else {
                    tracing::warn!("binary message too short: {} bytes", bytes.len());
                }
            }
            Message::Text(text) => tracing::info!("received text: {}", text),
            Message::Ping(_) => tracing::info!("received ping"),
            Message::Pong(_) => tracing::info!("received pong"),
            _ => {}
        }
    }
    Ok(())
}
