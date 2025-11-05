use ac::*;
use anyhow::Result;
use device::*;
use rand::Rng;
use std::io::Write;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use window::*;
use windows::Win32::Devices::HumanInterfaceDevice::{
    IDirectInput8W, IDirectInputDevice8W, IDirectInputEffect,
};

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
        loop {
            let result = std::panic::catch_unwind(|| {
                let hwnd = match create_window("Design & Solutions: FFB", "WindowClass") {
                    Ok(h) => h,
                    Err(e) => {
                        eprintln!("Failed to create window: {:?}", e);
                        return;
                    }
                };

                let di = match initialize_dirent_input() {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("Failed to initialize DirectInput: {:?}", e);
                        return;
                    }
                };

                loop {
                    let (_name, instance) = match found_device(&di) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("No FFB device found: {:?}", e);
                            std::thread::sleep(Duration::from_secs(1));
                            continue;
                        }
                    };

                    let device = match create_device(&di, instance, hwnd) {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Failed to create device: {:?}", e);
                            std::thread::sleep(Duration::from_secs(1));
                            continue;
                        }
                    };

                    let effect = match create_effect(&device) {
                        Ok(e) => e,
                        Err(e) => {
                            eprintln!("Failed to create effect: {:?}", e);
                            std::thread::sleep(Duration::from_secs(1));
                            continue;
                        }
                    };

                    if let Err(e) = apply_centering_spring(&effect) {
                        eprintln!("Failed to apply centering spring: {:?}", e);
                        std::thread::sleep(Duration::from_secs(1));
                        continue;
                    }

                    if let Err(e) = effect.Start(1, 0) {
                        eprintln!("Failed to start effect: {:?}", e);
                        std::thread::sleep(Duration::from_secs(1));
                        continue;
                    }

                    println!("FFB effect running...");

                    'effect_loop: loop {
                        match read_ac_data() {
                            Some(data) => {
                                if let Err(err) = update_effect(&effect, data.final_ff) {
                                    eprintln!("Update FFB failed: {:?}", err);
                                    break 'effect_loop; // reconnect
                                }
                            }
                            None => {
                                eprintln!("Read AC data failed, retrying...");
                                std::thread::sleep(Duration::from_secs(1));
                            }
                        }

                        std::thread::sleep(Duration::from_millis(1));
                        std::io::stdout().flush().unwrap();
                    }
                    println!("Cleaning up FFB device...");
                    let _ = effect.Stop();
                    let _ = device.Unacquire();
                    drop(effect);
                    drop(device);

                    println!("Restarting FFB setup...");
                    std::thread::sleep(Duration::from_secs(5));
                }
            });

            if result.is_err() {
                eprintln!("Unexpected panic occurred, restarting FFB loop...");
            }

            println!("Restarting FFB all...");
            std::thread::sleep(Duration::from_secs(5));
            std::io::stdout().flush().unwrap();
        }
    }
}
