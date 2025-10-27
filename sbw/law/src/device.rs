use anyhow::{Context, Result, anyhow};
use std::thread::sleep;
use std::time::Duration;
use std::{ffi::c_void, ptr::null_mut};

use windows::{
    Win32::{
        Devices::HumanInterfaceDevice::*,
        Foundation::HWND,
        System::{Com::*, LibraryLoader::*},
    },
    core::*,
};

use crate::DWORD;

pub unsafe fn initialize_dirent_input() -> Result<IDirectInput8W> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let hmodule = GetModuleHandleW(None)?;
        let mut di_ptr: *mut IDirectInput8W = null_mut();
        DirectInput8Create(
            hmodule.into(),
            DIRECTINPUT_VERSION,
            &IDirectInput8W::IID,
            &mut di_ptr as *mut _ as *mut _,
            None,
        )?;
        let di: IDirectInput8W = IDirectInput8W::from_raw(di_ptr as *mut _);

        println!("DirectInput initialized!");
        Ok(di)
    }
}

pub unsafe fn found_device(di: &IDirectInput8W) -> Result<(String, DIDEVICEINSTANCEW)> {
    unsafe {
        let mut devices: Vec<(String, DIDEVICEINSTANCEW)> = Vec::new();
        di.EnumDevices(
            DI8DEVCLASS_GAMECTRL,
            Some(enum_devices_callback),
            &mut devices as *mut _ as *mut c_void,
            DIEDFL_ATTACHEDONLY,
        )?;

        let device = devices
            .into_iter()
            .find(|(name, _)| name.contains("Simucube 2 Pro"))
            .ok_or_else(|| anyhow::anyhow!("Simucube 2 Pro not found"))?;
        println!("Simucube 2 Pro found");
        Ok(device)
    }
}

unsafe extern "system" fn enum_devices_callback(
    device_instance: *mut DIDEVICEINSTANCEW,
    context: *mut std::ffi::c_void,
) -> BOOL {
    unsafe {
        if !device_instance.is_null() {
            // tszInstanceName is a fixed-size WCHAR array
            let name_wstr = &(*device_instance).tszInstanceName;
            // Convert null-terminated WCHAR array to Rust String
            let len = name_wstr
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(name_wstr.len());
            let name = String::from_utf16_lossy(&name_wstr[..len]);
            println!("Device: {}", name);

            // Append to Vec<String> passed in context
            let vec_ptr = context as *mut Vec<(String, DIDEVICEINSTANCEW)>;
            if let Some(vec) = vec_ptr.as_mut() {
                vec.push((name, *device_instance));
            }
        }
    }
    BOOL(1)
}

pub unsafe fn create_device(
    di: &IDirectInput8W,
    instance: DIDEVICEINSTANCEW,
    hwnd: HWND,
) -> Result<IDirectInputDevice8W> {
    unsafe {
        let mut device_opt: Option<IDirectInputDevice8W> = None;
        di.CreateDevice(&instance.guidInstance, &mut device_opt, None)?;
        println!("Device is created");
        let device = match device_opt {
            Some(device) => device,
            None => {
                return Err(anyhow!("Cannot continue without device"));
            }
        };
        println!("Device has been found");
        device.SetCooperativeLevel(hwnd, DISCL_EXCLUSIVE | DISCL_FOREGROUND)?;
        unsafe extern "C" {
            static c_dfDIJoystick2: DIDATAFORMAT;
        }
        device.SetDataFormat(&c_dfDIJoystick2 as *const _ as *mut _)?;
        device.Acquire()?;
        println!("Acquire pass successfully");
        Ok(device)
    }
}

pub unsafe fn read_axis_x(device: &IDirectInputDevice8W) -> Result<f32> {
    unsafe {
        let mut state = DIJOYSTATE2::default();
        let res = device.GetDeviceState(
            std::mem::size_of::<DIJOYSTATE2>() as u32,
            &mut state as *mut _ as *mut _,
        );

        if res.is_err() {
            // try re-acquire if needed
            device.Acquire()?;
            return Ok(0.0);
        }

        // Normalize [-1.0, 1.0]
        let normalized = (state.lX as f32 - 32768.0) / 32768.0;
        let normalized = normalized.clamp(-1.0, 1.0);

        // Convert to degrees: [-450°, 450°]
        let degrees = normalized * 450.0;

        Ok(degrees)
    }
}
