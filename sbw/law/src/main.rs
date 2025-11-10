use ac::*;
use anyhow::Result;
use device::*;
use dotenvy::dotenv;
use rand::Rng;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use vjoy_sys::*;
use window::*;
use windows::Win32::Devices::HumanInterfaceDevice::{HID_USAGE_GENERIC_X, IDirectInputEffect};
use windows::Win32::Devices::HumanInterfaceDevice::{IDirectInput8W, IDirectInputDevice8W};

use crate::data::draw_steering_table;

mod ac;
mod data;
mod device;
mod model;
mod window;

#[derive(Debug)]
struct FFBData {
    speed_kmh: f32,
}

#[derive(Debug)]
pub struct SteeringTable {
    pub wheel_angles: [[f32; 14]; 13],
    pub key_steer_angle: [i32; 13],
    pub key_speed: [i32; 14],
    pub max_wheel_angle: f32,
    pub scalling_factor: f32,
}

impl SteeringTable {
    pub fn new(factor: f32, angle: f32, model: &str) -> Self {
        if model == "A424" {
            data::get_data_a424(factor, angle)
        } else {
            panic!("Wrong model")
        }
    }

    pub fn get_wheel_angle(&self, speed: f32, steer_angle: f32) -> f32 {
        let sign = steer_angle.signum(); // Save the original sign
        let steer_abs = steer_angle.abs() * self.scalling_factor; // Use absolute value for table lookup

        // Find closest speed indices in the key table
        let s_idx = match self
            .key_speed
            .binary_search_by(|v| v.partial_cmp(&(speed as i32)).unwrap())
        {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let s_next = (s_idx + 1).min(self.key_speed.len() - 1);
        let speed0 = self.key_speed[s_idx] as f32;
        let speed1 = self.key_speed[s_next] as f32;
        let t_speed = if speed1 - speed0 == 0.0 {
            0.0
        } else {
            (speed - speed0) / (speed1 - speed0)
        };

        // Find closest steer angle indices
        let a_idx = match self
            .key_steer_angle
            .binary_search_by(|v| v.partial_cmp(&(steer_abs as i32)).unwrap())
        {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let a_next = (a_idx + 1).min(self.key_steer_angle.len() - 1);
        let steer0 = self.key_steer_angle[a_idx] as f32;
        let steer1 = self.key_steer_angle[a_next] as f32;
        let t_steer = if steer1 - steer0 == 0.0 {
            0.0
        } else {
            (steer_abs - steer0) / (steer1 - steer0)
        };

        // Bilinear interpolation
        let v00 = self.wheel_angles[a_idx][s_idx];
        let v10 = self.wheel_angles[a_next][s_idx];
        let v01 = self.wheel_angles[a_idx][s_next];
        let v11 = self.wheel_angles[a_next][s_next];

        let interp_steer0 = v00 * (1.0 - t_steer) + v10 * t_steer;
        let interp_steer1 = v01 * (1.0 - t_steer) + v11 * t_steer;
        let wheel_angle = interp_steer0 * (1.0 - t_speed) + interp_steer1 * t_speed;

        wheel_angle * sign
    }
}

type DWORD = u32;
const HID_USAGE_X: u32 = 0x30;

fn main() -> Result<()> {
    loop {
        // Wrap everything in a catch_unwind to recover from panics
        let result = std::panic::catch_unwind(|| {
            dotenv().ok();
            let factor: f32 = std::env::var("FACTOR")
                .unwrap_or_else(|_| "1.0".to_string())
                .parse()
                .unwrap_or(1.0);
            let steer_total_angle: f32 = std::env::var("STEER_TOTAL_ANGLE")
                .unwrap_or_else(|_| "900.0".to_string())
                .parse()
                .unwrap_or(900.0);
            let steer_limit_angle: f32 = std::env::var("STEER_LIMIT_ANGLE")
                .unwrap_or_else(|_| "210.0".to_string())
                .parse()
                .unwrap_or(210.0);
            let model: String = std::env::var("CAR_MODEL").unwrap_or_else(|_| "A424".to_string());
            let table = SteeringTable::new(factor, steer_limit_angle, &model);

            let mut plots: Vec<Vec<f32>> = Vec::new();
            for steer_angle in table.key_steer_angle {
                let mut plot: Vec<f32> = Vec::new();
                for speed in table.key_speed {
                    let wheel_angle = table.get_wheel_angle(speed as f32, steer_angle as f32);
                    plot.push(wheel_angle);
                }
                plots.push(plot);
            }
            draw_steering_table(&table, plots, "steering_table.png");

            unsafe {
                let vjoy = match vJoyInterface::new(Path::new(
                    r"C:\Program Files\vJoy\x64\vJoyInterface.dll",
                )) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("vJoy init failed: {:?}", e);
                        return;
                    }
                };
                let device_id = 1;
                if vjoy.AcquireVJD(device_id) == 0 {
                    eprintln!("Failed to acquire vJoy device {}", device_id);
                    return;
                }

                let hwnd = match create_window("Design & Solutions: Law", "WindowClass") {
                    Ok(h) => h,
                    Err(e) => {
                        eprintln!("Window creation failed: {:?}", e);
                        return;
                    }
                };
                let di = match initialize_dirent_input() {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("DirectInput init failed: {:?}", e);
                        return;
                    }
                };

                loop {
                    let (_name, instance) = match found_device(&di) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Device not found: {:?}", e);
                            std::thread::sleep(Duration::from_secs(1));
                            continue;
                        }
                    };
                    let mut device = match create_device(&di, instance, hwnd) {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Failed to create device: {:?}", e);
                            std::thread::sleep(Duration::from_secs(1));
                            continue;
                        }
                    };

                    'effect_loop: loop {
                        match read_ac_data() {
                            Some(data) => {
                                if device.Acquire().is_err() {
                                    eprintln!("Wheel disconnected, restarting...");
                                    break 'effect_loop;
                                }

                                let speed = data.speed_kmh;
                                let steer_angle = match read_axis_x(&device, steer_total_angle) {
                                    Ok(a) => a,
                                    Err(_) => continue,
                                };
                                let wheel_angle = table.get_wheel_angle(speed, steer_angle);

                                let normalized =
                                    (wheel_angle / table.max_wheel_angle).clamp(-1.0, 1.0);
                                let vjoy_value = float_to_vjoy_axis(normalized);

                                if vjoy.SetAxis(vjoy_value as i32, device_id as u32, HID_USAGE_X)
                                    == 0
                                {
                                    eprintln!("Failed to set vJoy axis");
                                }

                                print!("\x1B[2J\x1B[1;1H");
                                println!(
                                    "\rCar Speed: {:3.0} km/h | Steer Angle: {:3.1}° | Target Wheel Angle: {:3.1}° | vJoy value: {:1.2}",
                                    speed, steer_angle, wheel_angle, normalized,
                                );
                            }
                            None => {
                                print!("\x1B[2J\x1B[1;1H");
                                println!("Read AC data failed, retrying...");
                                std::thread::sleep(Duration::from_millis(500));
                            }
                        }
                        std::thread::sleep(Duration::from_millis(3));
                        std::io::stdout().flush().unwrap();
                    }

                    println!("Restarting LAW setup...");
                    std::thread::sleep(Duration::from_secs(5));
                    std::io::stdout().flush().unwrap();
                }
            }
        });

        if result.is_err() {
            eprintln!("Unexpected panic occurred, restarting main loop...");
        }

        println!("Restarting LAW all...");
        std::thread::sleep(Duration::from_secs(5));
        std::io::stdout().flush().unwrap();
    }
}

unsafe fn float_to_vjoy_axis(value: f32) -> u32 {
    let clamped = value.clamp(-1.0, 1.0);
    ((clamped + 1.0) / 2.0 * 32768 as f32) as u32
}
