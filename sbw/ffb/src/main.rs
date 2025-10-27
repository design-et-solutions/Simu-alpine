use ac::*;
use anyhow::Result;
use device::*;
use rand::Rng;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use window::*;
use windows::Win32::Devices::HumanInterfaceDevice::{IDirectInputDevice8W, IDirectInputEffect};

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
        let effect = create_effect(&device)?;
        apply_centering_spring(&effect)?;
        effect.Start(1, 0)?;
        println!("FFB effect running...");
        loop_ac(&device, &effect)?;
        println!("FFB effect stopping...");
        Ok(())
    }
}

unsafe fn loop_ac(device: &IDirectInputDevice8W, effect: &IDirectInputEffect) -> Result<()> {
    loop {
        match read_ac_data() {
            Some(data) => unsafe {
                if let Err(err) = update_effect(effect, data.final_ff) {
                    println!("Update FFB failed: {:?}", err);
                }
            },
            None => {
                unsafe {
                    let steer_angle = read_axis_x(device)?;
                    println!("Steer angle: {}", steer_angle);
                }
                println!("Read AC data failed");
            }
        }
        std::thread::sleep(Duration::from_micros(3000));
    }
}
