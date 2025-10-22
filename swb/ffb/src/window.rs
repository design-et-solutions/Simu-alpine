use super::AC_DATA;
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

pub unsafe fn create_window(title: &str, class: &str, is_fg: bool) -> Result<HWND> {
    let class_w: Vec<u16> = class.encode_utf16().chain(std::iter::once(0)).collect();
    let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        let wc = WNDCLASSW {
            lpfnWndProc: Some(if is_fg {
                window_proc_fg
            } else {
                window_proc_bg
            }),
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
            None,
        )?;
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);
        let _ = SetForegroundWindow(hwnd);
        let _ = SetFocus(Some(hwnd));
        Ok(hwnd)
    }
}

unsafe extern "system" fn window_proc_bg(
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

unsafe extern "system" fn window_proc_fg(
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
                let ac_data = *AC_DATA.lock().unwrap();
                let text = format!("FFB Torque: {:.2}", ac_data.finalFF);
                let wtext: Vec<u16> = text.encode_utf16().collect();
                let _ = TextOutW(hdc, 10, 10, &wtext);
                // let text = format!("Steering Angle: {:.2}", ac_data.steerAngle);
                // let wtext: Vec<u16> = text.encode_utf16().collect();
                // let _ = TextOutW(hdc, 10, 60, &wtext);
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
