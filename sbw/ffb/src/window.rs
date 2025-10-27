use std::sync::{Arc, Mutex};

use anyhow::Result;
use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::LibraryLoader::*,
        UI::Input::KeyboardAndMouse::*,
        UI::WindowsAndMessaging::{DefWindowProcW, WNDCLASSW, *},
    },
    core::*,
};

pub unsafe fn create_window(title: &str, class: &str) -> Result<HWND> {
    let class_w: Vec<u16> = class.encode_utf16().chain(std::iter::once(0)).collect();
    let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance.into(),
            lpszClassName: PCWSTR(class_w.as_ptr()),
            ..Default::default()
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            Default::default(),
            PCWSTR(class_w.as_ptr()),
            PCWSTR(title_w.as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            400,
            200,
            None,
            None,
            Some(hinstance.into()),
            None, // Some(Arc::into_raw(data.clone()) as *const std::ffi::c_void),
        )?;
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);
        let _ = SetForegroundWindow(hwnd);
        let _ = SetFocus(Some(hwnd));
        println!("Window has been setup");
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
            WM_NCCREATE => {
                let createstruct = lparam.0 as *const CREATESTRUCTW;
                let data_ptr = (*createstruct).lpCreateParams as *mut Arc<Mutex<f32>>;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, data_ptr as isize);
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_PAINT => LRESULT(0),
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
