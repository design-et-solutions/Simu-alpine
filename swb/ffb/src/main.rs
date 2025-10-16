use anyhow::{Context, Result};
use std::ffi::c_void;
use std::ptr::null_mut;
use windows::{
    Win32::{
        Devices::HumanInterfaceDevice::*,
        System::{Com::*, Console::*, LibraryLoader::*, Threading::*},
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

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
            println!("Found device: {}", name);

            // Append to Vec<String> passed in context
            let vec_ptr = context as *mut Vec<(String, DIDEVICEINSTANCEW)>;
            if let Some(vec) = vec_ptr.as_mut() {
                vec.push((name, *device_instance));
            }
        }
    }
    BOOL(1)
}

fn main() -> Result<()> {
    unsafe {
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
            println!("--------------------");
            println!("{:?}", instance);
            println!("--------------------");

            println!("Create device");
            let mut device_opt: Option<IDirectInputDevice8W> = None;
            di.CreateDevice(&instance.guidInstance, &mut device_opt, None)
                .context("Failed to create device")?;
            let device = device_opt.context("Device not found")?;

            println!("Configure device");
            // AllocConsole()?;
            // let hwnd = GetConsoleWindow();
            let hwnd = CreateWindowExW(
                Default::default(),
                w!("STATIC"),
                w!("FFB Test Window"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                100,
                100,
                None,
                None,
                None,
                None,
            )?;
            println!("Console HWND = {:?}", hwnd);
            device.SetCooperativeLevel(hwnd, DISCL_FOREGROUND | DISCL_EXCLUSIVE)?;
            device.Acquire()?;

            println!("Create FFB effect");
            let direction: [i32; 2] = [0, 0];
            let constant_force = DICONSTANTFORCE {
                lMagnitude: 5000, // out of ±10 000 range — tune this
            };
            let mut axes: [u32; 1] = [0];
            let mut effect = DIEFFECT {
                dwSize: std::mem::size_of::<DIEFFECT>() as u32,
                dwFlags: DIEFF_CARTESIAN | DIEFF_OBJECTOFFSETS,
                dwDuration: INFINITE,
                dwGain: 10_000, // max gain
                cAxes: 1,
                rgdwAxes: axes.as_mut_ptr(),
                rglDirection: direction.as_ptr() as *mut i32,
                lpEnvelope: null_mut(),
                cbTypeSpecificParams: std::mem::size_of::<DICONSTANTFORCE>() as u32,
                lpvTypeSpecificParams: &constant_force as *const _ as *mut c_void,
                dwTriggerButton: DIEB_NOTRIGGER,
                dwSamplePeriod: 0,
                dwStartDelay: 0,
                dwTriggerRepeatInterval: 0,
            };
            let mut effect_ptr: Option<IDirectInputEffect> = None;
            device.CreateEffect(&GUID_ConstantForce, &mut effect, &mut effect_ptr, None)?;
            println!("Effect created!");

            if let Some(eff) = effect_ptr {
                eff.Download()?;
                println!("Effect downloaded.");

                eff.Start(1, 0)?; // 1 iteration, 0 flags
                println!("Effect started! The wheel should now be centered.");
            }
        } else {
            println!("Simucube 2 Pro not found");
        }

        use std::time::Duration;
        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
