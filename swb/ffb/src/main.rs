use ac::read_ac_data;
use anyhow::Result;
use device::initialize_dirent_input;
use std::panic::catch_unwind;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use window::create_window;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::{Foundation::*, Graphics::Gdi::*};

use crate::device::{create_device, create_effect, found_device, run_effect};

mod ac;
mod device;
mod model;
mod window;

#[derive(Debug, Clone, Copy, Default)]
pub struct FFBData {
    pub steerAngle: f32,
    pub finalFF: f32,
}

type DWORD = u32;

lazy_static::lazy_static! {
    static ref FFB_AXES: Mutex<Vec<DWORD>> = Mutex::new(Vec::new());
}

lazy_static::lazy_static! {
    static ref AC_DATA: Arc<Mutex<FFBData>> = Arc::new(Mutex::new(FFBData::default()));
}

fn loop_ac_data() {
    println!("Thread: AC_DATA Start");
    loop {
        if let Some(ac_data) = read_ac_data() {
            println!("{:?}", ac_data);
            *AC_DATA.lock().unwrap() = ac_data
        } else {
            println!("❌ Failed to get AC Data");
        }
        // std::thread::sleep(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

unsafe fn loop_ffb() {
    println!("Thread: FFB Start");
    unsafe {
        let hwnd = create_window("Design & Solutions: FFB (BG)", "FFBWindowClassBG", false);
        println!("Window BG has been setup");
        std::thread::sleep(std::time::Duration::from_secs(1));

        let di = initialize_dirent_input();

        println!("--------------------");

        let device = if let Some((name, instance)) = found_device(di.clone()) {
            println!("{name} found");
            let device = create_device(di, instance, hwnd);
            device
        } else {
            panic!("Simucube 2 Pro not found");
        };

        println!("--------------------");

        loop {
            let result = catch_unwind(|| {
                let effect_ptr = create_effect(device.clone());
                run_effect(effect_ptr);
            });

            if result.is_err() {
                println!("❌ FFB effect panicked, recovering...");
            }

            std::thread::sleep(std::time::Duration::from_secs(15));
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        std::thread::spawn(loop_ac_data);
        std::thread::spawn(|| {
            loop {
                let _ = catch_unwind(|| loop_ffb());
                println!("❌ FFB thread panicked, restarting...");
                std::thread::sleep(Duration::from_secs(1)); // avoid rapid restart
            }
        });

        let hwnd = create_window("Design & Solutions: FFB (FG)", "FFBWindowClassFG", true);
        println!("Window FG has been setup");
        let mut msg = MSG::default();
        loop {
            while PeekMessageW(&mut msg, Some(HWND(std::ptr::null_mut())), 0, 0, PM_REMOVE).into() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            // Trigger repaint so WM_PAINT shows latest FFB
            let _ = InvalidateRect(Some(hwnd), None, true);

            // Sleep a bit to avoid busy loop
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
