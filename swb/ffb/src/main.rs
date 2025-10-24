use ac::*;
use anyhow::Result;
use device::*;
use rand::Rng;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use window::*;
use windows::Win32::Devices::HumanInterfaceDevice::IDirectInputEffect;

type DWORD = u32;

lazy_static::lazy_static! {
    static ref FFB_AXES: Mutex<Vec<DWORD>> = Mutex::new(Vec::new());
}

mod ac;
mod device;
mod model;
mod window;

#[derive(Debug)]
struct FFBData {
    final_ff: f32,
    steer_angle: f32,
}

fn main() -> Result<()> {
    unsafe {
        let hwnd = create_window("Design & Solutions: FFB", "WindowClass")?;
        let di = initialize_dirent_input()?;
        let (_name, instance) = found_device(&di)?;
        let device = create_device(&di, instance, hwnd)?;
        // let effect = create_effect(&device)?;
        // effect.Start(1, 0)?;

        let constant = create_constant_force(&device)?;
        // let spring = create_spring(&device)?;
        // let damper = create_damper(&device)?;

        constant.Start(1, 0)?;
        // spring.Start(1, 0)?;
        // damper.Start(1, 0)?;

        println!("FFB effect running...");
        // loop_ac(&constant, &spring, &damper)?;
        loop_ac(&constant)?;
        // loop_ac(&effect)?;
        // loop_rand(&effect)?;
        println!("FFB effect stopping...");
        Ok(())
    }
}

unsafe fn loop_ac(
    constant: &IDirectInputEffect,
    // spring: &IDirectInputEffect,
    // damper: &IDirectInputEffect,
) -> Result<()> {
    let mut prev_force = 0.0f32;
    let mut prev_steer = 0.0f32;
    let mut last_time = std::time::Instant::now();

    loop {
        match read_ac_data() {
            Some(data) => {
                println!("Current data: {:?}", data);
                let now = std::time::Instant::now();
                let delta = now.duration_since(last_time).as_secs_f32();
                last_time = now;

                let steer_angle = data.steer_angle;
                let steer_velocity = if delta > 0.0 {
                    (steer_angle - prev_steer) / delta
                } else {
                    0.0
                };
                prev_steer = steer_angle;

                let alpha = 0.2;
                let smoothed = prev_force + alpha * (data.final_ff - prev_force);
                prev_force = smoothed;

                let shaped = smoothed.signum() * smoothed.abs().powf(1.8);

                unsafe {
                    if let Err(err) = update_effect(constant, shaped) {
                        println!("Constant force update failed: {:?}", err);
                    }
                    // if let Err(err) = update_spring(spring, steer_angle) {
                    //     println!("Spring update failed: {:?}", err);
                    // }

                    // if let Err(err) = update_damper(damper, steer_velocity) {
                    //     println!("Damper update failed: {:?}", err);
                    // }
                    // if let Err(err) = update_effect(spring, curved) {
                    //     println!("Update FFB failed: {:?}", err);
                    // }
                    // if let Err(err) = update_effect(damper, curved) {
                    //     println!("Update FFB failed: {:?}", err);
                    // }
                }
            }
            None => {
                println!("Read AC data failed");
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

// unsafe fn loop_ac(effect: &IDirectInputEffect) -> Result<()> {
//     let mut prev = 0.0;
//     loop {
//         match read_ac_data() {
//             Some(data) => {
//                 println!("Current FFB torque: {:.2}", data.final_ff);
//                 unsafe {
//                     let alpha = 0.15;
//                     let smoothed = prev + alpha * (data.final_ff - prev);
//                     prev = smoothed;
//                     let curved = smoothed.signum() * smoothed.abs().powf(1.8);
//                     if let Err(err) = update_effect(effect, curved) {
//                         println!("Update FFB failed: {:?}", err);
//                     }
//                 }
//             }
//             None => {
//                 println!("Read AC data failed");
//             }
//         }
//         std::thread::sleep(Duration::from_millis(20));
//     }
// }

fn shape_force(x: f32) -> f32 {
    let clipped = x.clamp(-1.0, 1.0);
    let curved = clipped.signum() * clipped.abs().powf(1.8); // tweak the exponent
    curved * 0.9 // gain
}
