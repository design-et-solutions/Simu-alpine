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

pub unsafe fn update_effect(effect: &IDirectInputEffect, magnitude: f32) -> Result<()> {
    unsafe {
        let scaled = (magnitude.clamp(-1.0, 1.0) * 10000.0) as i32;

        let mut constant_force = DICONSTANTFORCE { lMagnitude: scaled };

        let mut dieffect = DIEFFECT {
            dwSize: std::mem::size_of::<DIEFFECT>() as u32,
            cbTypeSpecificParams: std::mem::size_of::<DICONSTANTFORCE>() as u32,
            lpvTypeSpecificParams: &mut constant_force as *mut _ as *mut std::ffi::c_void,
            dwFlags: DIEFF_CARTESIAN,
            ..Default::default()
        };

        effect.SetParameters(&mut dieffect, DIEP_TYPESPECIFICPARAMS)?;
        Ok(())
    }
}

pub unsafe fn create_effect(device: &IDirectInputDevice8W) -> Result<IDirectInputEffect> {
    unsafe {
        let mut axes = [0];
        let mut direction = Box::new([0i32]);
        let mut constant_force = Box::new(DICONSTANTFORCE { lMagnitude: 0 });

        let mut effect = DIEFFECT {
            dwSize: std::mem::size_of::<DIEFFECT>() as DWORD,
            dwFlags: DIEFF_CARTESIAN | DIEFF_OBJECTOFFSETS,
            dwDuration: 0xFFFFFFFF,
            dwGain: 10000,
            dwTriggerButton: DIEB_NOTRIGGER,
            dwTriggerRepeatInterval: 0,
            cAxes: 1,
            rgdwAxes: axes.as_mut_ptr(),
            rglDirection: direction.as_mut_ptr(),
            lpEnvelope: std::ptr::null_mut(),
            cbTypeSpecificParams: std::mem::size_of::<DICONSTANTFORCE>() as DWORD,
            lpvTypeSpecificParams: &mut constant_force as *mut _ as *mut c_void,
            ..Default::default()
        };

        println!("Effect parameters: {:#?}", effect);

        let mut effect_ptr: Option<IDirectInputEffect> = None;
        device.CreateEffect(&GUID_ConstantForce, &mut effect, &mut effect_ptr, None)?;
        let effect = effect_ptr.ok_or_else(|| anyhow::anyhow!("Failed to create effect"))?;

        println!("Effect created successfully!");
        Ok(effect)
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

pub unsafe fn apply_centering_spring(effect: &IDirectInputEffect) -> Result<()> {
    unsafe {
        let mut condition = DICONDITION {
            lOffset: 0, // zero at center
            lPositiveCoefficient: 10000,
            lNegativeCoefficient: 10000,
            lDeadBand: 0,
            dwPositiveSaturation: 10000,
            dwNegativeSaturation: 10000,
        };

        let mut dieffect = DIEFFECT {
            dwSize: std::mem::size_of::<DIEFFECT>() as u32,
            cbTypeSpecificParams: std::mem::size_of::<DICONSTANTFORCE>() as u32,
            lpvTypeSpecificParams: &mut condition as *mut _ as *mut _,
            dwFlags: DIEFF_CARTESIAN,
            ..Default::default()
        };

        effect.SetParameters(&mut dieffect, DIEP_TYPESPECIFICPARAMS)?;
    }
    Ok(())
}
