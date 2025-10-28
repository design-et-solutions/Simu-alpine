use ac::*;
use anyhow::Result;
use device::*;
use rand::Rng;
use std::io::Write;
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
        loop {
            match read_ac_data() {
                Some(data) => {
                    if let Err(err) = update_effect(&effect, data.final_ff) {
                        print!("\x1B[2J\x1B[1;1H"); // ANSI escape: clear screen + move cursor to top-left
                        println!("Update FFB failed: {:?}", err);
                        std::io::stdout().flush().unwrap();
                    }
                }
                None => {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("Read AC data failed");
                    std::io::stdout().flush().unwrap();
                }
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
