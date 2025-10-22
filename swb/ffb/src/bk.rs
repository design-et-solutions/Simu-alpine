use anyhow::{Context, Result};
use std::ffi::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{ffi::CString, mem, ptr};
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

lazy_static::lazy_static! {
    static ref FFB_VALUE: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SPageFilePhysics {
    pub packetId: i32,
    pub gas: f32,
    pub brake: f32,
    pub fuel: f32,
    pub gear: i32,
    pub rpms: i32,
    pub steerAngle: f32,
    pub speedKmh: f32,
    pub velocity: [f32; 3],
    pub accG: [f32; 3],
    pub wheelSlip: [f32; 4],
    pub wheelLoad: [f32; 4],
    pub wheelsPressure: [f32; 4],
    pub wheelAngularSpeed: [f32; 4],
    pub tyreWear: [f32; 4],
    pub tyreDirtyLevel: [f32; 4],
    pub tyreCoreTemperature: [f32; 4],
    pub camberRAD: [f32; 4],
    pub suspensionTravel: [f32; 4],
    pub drs: f32,
    pub tc: f32,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub cgHeight: f32,
    pub carDamage: [f32; 5],
    pub numberOfTyresOut: i32,
    pub pitLimiterOn: i32,
    pub abs: f32,
    pub kersCharge: f32,
    pub kersInput: f32,
    pub autoShifterOn: i32,
    pub rideHeight: [f32; 2],
    pub turboBoost: f32,
    pub ballast: f32,
    pub airDensity: f32,
    pub airTemp: f32,
    pub roadTemp: f32,
    pub localAngularVel: [f32; 3],
    pub finalFF: f32,
    pub performanceMeter: f32,
    pub engineBrake: i32,
    pub ersRecoveryLevel: i32,
    pub ersPowerLevel: i32,
    pub ersHeatCharging: i32,
    pub ersIsCharging: i32,
    pub kersCurrentKJ: f32,
    pub drsAvailable: i32,
    pub drsEnabled: i32,
    pub brakeTemp: [f32; 4],
    pub clutch: f32,
    pub tyreTempI: [f32; 4],
    pub tyreTempM: [f32; 4],
    pub tyreTempO: [f32; 4],
    pub isAIControlled: i32,
    pub tyreContactPoint: [[f32; 3]; 4],
    pub tyreContactNormal: [[f32; 3]; 4],
    pub tyreContactHeading: [[f32; 3]; 4],
    pub brakeBias: f32,
    pub localVelocity: [f32; 3],
    pub P2PActivations: i32,
    pub P2PStatus: i32,
    pub currentMaxRpm: i32,
}

fn read_ac_steer_torque() -> Option<f32> {
    unsafe {
        let name = CString::new("Local\\acpmf_physics").unwrap();
        match OpenFileMappingA(FILE_MAP_READ.0, false, PCSTR(name.as_ptr() as *const u8)) {
            Ok(mapping) => {
                let ptr = MapViewOfFile(
                    mapping,
                    FILE_MAP_READ,
                    0,
                    0,
                    mem::size_of::<SPageFilePhysics>(),
                );
                if ptr.Value.is_null() {
                    eprintln!("Failed to map view of file");
                    return None;
                }
                let physics: &SPageFilePhysics = &*(ptr.Value as *const SPageFilePhysics);
                let torque = physics.finalFF;
                if let Err(err) = UnmapViewOfFile(ptr as _) {
                    eprintln!("Failed to unmap view of file: {:?}", err);
                }
                Some(torque)
            }
            Err(err) => {
                eprintln!("Failed to open shared memory mapping: {:?}", err);
                None
            }
        }
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

unsafe fn create_window() -> Result<HWND> {
    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance.into(),
            lpszClassName: w!("WindowClass"),
            ..Default::default()
        };
        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            Default::default(),
            w!("WindowClass"),
            w!("Design & Solutions: FFB"),
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

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                // read the latest FFB value
                let torque = *FFB_VALUE.lock().unwrap(); // FFB_VALUE is your global Arc<Mutex<f32>>

                let text = format!("FFB Torque: {:.2}", torque);
                let wtext: Vec<u16> = text.encode_utf16().collect(); // no null terminator needed
                let _ = TextOutW(hdc, 10, 10, &wtext); // <- pass slice directly
                let _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        let hwnd = create_window()?;
        println!("Window has been setup");

        let ffb_value = Arc::new(Mutex::new(0.0f32));
        {
            let ffb_value = ffb_value.clone();
            std::thread::spawn(move || {
                loop {
                    if let Some(torque) = read_ac_steer_torque() {
                        *ffb_value.lock().unwrap() = torque;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            });
        }

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

            loop {
                if let Some(torque) = read_ac_steer_torque() {
                    println!("Current FFB torque: {:.2}", torque);
                    let text = format!("Simucube Test - FFB: {:.2}", torque);
                    let wtext: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
                    SetWindowTextW(hwnd, PCWSTR(wtext.as_ptr()))?;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        } else {
            println!("Simucube 2 Pro not found");
        }
        Ok(())
    }
}
