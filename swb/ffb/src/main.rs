use anyhow::{Context, Result};
use std::ffi::c_void;
use std::ptr::null_mut;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use windows::{
    Win32::{
        Devices::HumanInterfaceDevice::*,
        Foundation::*,
        Graphics::Gdi::*,
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

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

unsafe fn create_message_window() -> Result<HWND> {
    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance.into(),
            lpszClassName: w!("DummyWindowClass"),
            ..Default::default()
        };
        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            Default::default(),
            w!("DummyWindowClass"),
            w!("Simucube Test"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            400,
            200,
            None,
            None,
            Some(hinstance.into()),
            None,
        )?;
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);
        let _ = SetForegroundWindow(hwnd);
        let _ = SetFocus(Some(hwnd));
        Ok(hwnd)
    }
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
            // println!("--------------------");
            // println!("{:?}", instance);
            println!("--------------------");

            println!("Create device");
            let mut device_opt: Option<IDirectInputDevice8W> = None;
            di.CreateDevice(&instance.guidInstance, &mut device_opt, None)
                .context("Failed to create device")?;
            let device = device_opt.context("Device not found")?;
            println!("Setup Window");
            let hwnd = create_message_window()?;
            println!("Console HWND = {:?}", hwnd);

            sleep(Duration::from_secs(1));

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
            let mut constant_force = Box::new(DICONSTANTFORCE { lMagnitude: 5000 });

            let mut effect = DIEFFECT {
                dwSize: std::mem::size_of::<DIEFFECT>() as DWORD,
                dwFlags: DIEFF_CARTESIAN | DIEFF_OBJECTOFFSETS,
                dwDuration: 10_000_000,
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
            eff.Start(1, 0)?;
            println!("FFB effect running...");

            println!("FFB effect: {:#?}", eff);
            sleep(Duration::from_secs(2));

            eff.Stop()?;
            println!("FFB effect stopped.");
        } else {
            println!("Simucube 2 Pro not found");
        }
        Ok(())
    }
}
