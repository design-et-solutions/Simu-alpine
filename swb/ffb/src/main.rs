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

struct FFBData {
    finalFF: f32,
}

fn main() -> Result<()> {
    unsafe {
        let hwnd = create_window("Design & Solutions: FFB", "WindowClass")?;
        let di = initialize_dirent_input()?;
        let (_name, instance) = found_device(&di)?;
        let device = create_device(&di, instance, hwnd)?;
        let effect = create_effect(&device)?;
        effect.Start(1, 0)?;
        println!("FFB effect running...");

        // effect.SetParameters(&mut params, DIEP_TYPESPECIFICPARAMS | DIEP_DIRECTION);

        loop_ac(&effect)?;
        // loop_rand(&effect)?;
        Ok(())
    }
}

fn loop_ac(effect: &IDirectInputEffect) -> Result<()> {
    loop {
        if let Ok(data) = read_ac_data() {
            println!("Current FFB torque: {:.2}", data.finalFF);
            unsafe {
                update_effect(effect, data.finalFF)?;
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn loop_rand(effect: &IDirectInputEffect) -> Result<()> {
    let start = Instant::now();
    let mut rng = rand::rng();

    loop {
        let t = start.elapsed().as_secs_f32();
        let base = (t * 0.5).sin(); // adjust 0.5 → lower = slower road curve changes
        let noise: f32 = rng.random_range(-0.05..0.05);
        let amplitude = 2.0 + (t * 0.1).sin(); // torque envelope (can vary slowly)
        let final_ff = base * amplitude + noise;
        println!("Simulated FFB torque: {:.2}", final_ff);
        unsafe {
            update_effect(effect, final_ff)?;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}
