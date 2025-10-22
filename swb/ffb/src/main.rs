use ac::*;
use anyhow::{Context, Result};
use device::*;
use model::*;
use std::ffi::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{ffi::CString, mem, ptr};
use window::*;
use windows::Win32::System::Threading::INFINITE;
use windows::{
    Win32::{
        Devices::HumanInterfaceDevice::*,
        Foundation::*,
        Graphics::Gdi::*,
        System::Memory::*,
        System::{Com::*, LibraryLoader::*},
        UI::Input::KeyboardAndMouse::*,
        UI::WindowsAndMessaging::{DefWindowProcW, WNDCLASSW, *},
    },
    core::*,
};

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
        let hwnd = create_window("Design & Solutions: FFB", "WindowClass", true)?;
        println!("Window has been setup");

        // Initialize COM
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();

        // Get module handle
        let hmodule = GetModuleHandleW(None).unwrap();

        // Prepare Option<IDirectInput8W> to receive the COM object
        let mut di_ptr: *mut IDirectInput8W = null_mut();

        // Call DirectInput8Create
        DirectInput8Create(
            hmodule.into(),
            DIRECTINPUT_VERSION,
            &IDirectInput8W::IID,
            &mut di_ptr as *mut _ as *mut _,
            None,
        )?;

        // Wrap it safely
        let di: IDirectInput8W = IDirectInput8W::from_raw(di_ptr as *mut _);

        println!("DirectInput initialized!");

        // Enumerate attached game controllers (DI8DEVCLASS_GAMECTRL)
        let mut devices: Vec<(String, DIDEVICEINSTANCEW)> = Vec::new();
        println!("--------------------");
        di.EnumDevices(
            DI8DEVCLASS_GAMECTRL,
            Some(enum_devices_callback),
            &mut devices as *mut _ as *mut c_void,
            DIEDFL_ATTACHEDONLY,
        )?;
        println!("--------------------");

        if let Some((_name, instance)) = devices
            .iter()
            .find(|(name, _)| name.contains("Simucube 2 Pro"))
        {
            println!("Simucube 2 Pro found");
            // println!("--------------------");
            // println!("{:?}", instance);
            println!("--------------------");

            println!("Create device");
            let mut device_opt: Option<IDirectInputDevice8W> = None;
            di.CreateDevice(&instance.guidInstance, &mut device_opt, None)
                .context("Failed to create device")?;
            let device = device_opt.context("Device not found")?;
            device.SetCooperativeLevel(hwnd, DISCL_EXCLUSIVE | DISCL_FOREGROUND)?;
            unsafe extern "C" {
                static c_dfDIJoystick2: DIDATAFORMAT;
            }
            device.SetDataFormat(&c_dfDIJoystick2 as *const _ as *mut _)?;
            device.Acquire()?;
            println!("Acquire pass successfully");

            println!("Create FFB effect");
            let mut axis = [0];
            let mut direction = Box::new([1i32]);
            let mut constant_force = Box::new(DICONSTANTFORCE { lMagnitude: 0 });

            let mut effect = DIEFFECT {
                dwSize: std::mem::size_of::<DIEFFECT>() as DWORD,
                dwFlags: DIEFF_CARTESIAN | DIEFF_OBJECTOFFSETS,
                dwDuration: INFINITE,
                dwGain: 5000,
                dwTriggerButton: DIEB_NOTRIGGER,
                dwTriggerRepeatInterval: 0,
                cAxes: 1,
                rgdwAxes: axis.as_mut_ptr(),
                rglDirection: direction.as_mut_ptr(),
                lpEnvelope: std::ptr::null_mut(),
                cbTypeSpecificParams: std::mem::size_of::<DICONSTANTFORCE>() as DWORD,
                lpvTypeSpecificParams: &mut constant_force as *mut _ as *mut c_void,
                ..Default::default()
            };
            println!("Effect parameters: {:#?}", effect);

            let mut effect_ptr: Option<IDirectInputEffect> = None;
            device.CreateEffect(&GUID_ConstantForce, &mut effect, &mut effect_ptr, None)?;
            println!("Effect created successfully!");

            let eff = effect_ptr.unwrap();
            eff.Start(1, DIEP_START | DIEP_NORESTART)?;
            println!("FFB effect running...");

            loop {
                if let Ok(data) = read_ac_data() {
                    println!("Current FFB torque: {:.2}", data.finalFF);
                    let text = format!("Simucube Test - FFB: {:.2}", data.finalFF);
                    let wtext: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
                    SetWindowTextW(hwnd, PCWSTR(wtext.as_ptr()))?;

                    constant_force.lMagnitude = (data.finalFF * 5000.0) as i32; // scale to device

                    let mut params = DIEFFECT {
                        dwSize: std::mem::size_of::<DIEFFECT>() as DWORD,
                        lpvTypeSpecificParams: &mut constant_force as *mut _ as *mut _,
                        cbTypeSpecificParams: std::mem::size_of::<DICONSTANTFORCE>() as DWORD,
                        dwFlags: DIEFF_CARTESIAN
                            | DIEFF_OBJECTOFFSETS
                            | DIEP_DIRECTION
                            | DIEP_TYPESPECIFICPARAMS,
                        ..Default::default()
                    };

                    let _ =
                        eff.SetParameters(&mut params, DIEP_TYPESPECIFICPARAMS | DIEP_DIRECTION);
                }
                std::thread::sleep(Duration::from_millis(20));
            }

            // println!("FFB effect: {:#?}", eff);
            // sleep(Duration::from_secs(2));

            // eff.Stop()?;
            // println!("FFB effect stopped.");

            // loop {
            //     if let Ok(torque) = ffb_value.lock() {
            //         println!("Current FFB torque: {:.2}", torque);
            //         let text = format!("Simucube Test - FFB: {:.2}", torque);
            //         let wtext: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
            //         SetWindowTextW(hwnd, PCWSTR(wtext.as_ptr()))?;
            //     }
            //     std::thread::sleep(std::time::Duration::from_millis(50));
            // }
        } else {
            println!("Simucube 2 Pro not found");
        }
        Ok(())
    }
}
