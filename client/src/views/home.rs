use crate::Route;
use crate::TelemetryCtx;
use crate::api::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let TelemetryCtx(telemetry_ctx) = use_context::<TelemetryCtx>();
    let _ = use_resource(|| async { get_ws().await });

    if let Some(telemetry) = telemetry_ctx() {
        let speed_ms = telemetry.speed_ms;
        let sp_line_length = telemetry.sp_line_length;
        let engine_life_left = telemetry.engine_life_left;
        let current_lap_time_ms = telemetry.current_lap_time_ms;
        let lap_count = telemetry.lap_count;
        let best_lap_ms = telemetry.best_lap_ms;
        let name_bytes: Vec<u8> = telemetry
            .car_model
            .iter()
            .copied()
            .take_while(|&c| c != 0)
            .collect();
        let car_model = String::from_utf8_lossy(&name_bytes);
        rsx! {
            div {
                font_weight: "700",
                font_size: "40px",
                display: "flex",
                flex_direction: "column",
                gap: "1rem",
                div { "car model: {car_model}" }
                div { "speed: {speed_ms}" }
                div { "track segment: {sp_line_length}" }
                div { "engine life left: {engine_life_left}" }
                div { "current lap time: {current_lap_time_ms}" }
                div { "lap count: {lap_count}" }
                div { "best lap: {best_lap_ms}" }
            }
        }
    } else {
        rsx! {
            div { "Should see telemetry data..." }
        }
    }
}
