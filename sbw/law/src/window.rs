use anyhow::Result;
use image::GenericImageView;
use std::sync::{Arc, Mutex};
use windows::Win32::Graphics::Gdi::*;
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
            1200,
            800,
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

static mut HIMG: HBITMAP = HBITMAP(std::ptr::null_mut());

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_CREATE => {
                HIMG = load_image_as_hbitmap("steering_table.png");
                LRESULT(0)
            }
            WM_NCCREATE => {
                let createstruct = lparam.0 as *const CREATESTRUCTW;
                let data_ptr = (*createstruct).lpCreateParams as *mut Arc<Mutex<f32>>;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, data_ptr as isize);
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                if HIMG.0 != std::ptr::null_mut() {
                    let hdc_mem = CreateCompatibleDC(Some(hdc));
                    let old_bmp = SelectObject(hdc_mem, HIMG.into());

                    BitBlt(hdc, 0, 0, 400, 200, Some(hdc_mem), 0, 0, SRCCOPY);

                    SelectObject(hdc_mem, old_bmp);
                    DeleteDC(hdc_mem);
                }

                EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                if HIMG.0 != std::ptr::null_mut() {
                    DeleteObject(HIMG.into());
                }
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

unsafe fn load_image_as_hbitmap(path: &str) -> HBITMAP {
    unsafe {
        let img = image::open(path).expect("Failed to open image");
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // negative for top-down DIB
                biPlanes: 1,
                biBitCount: 24,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            bmiColors: [Default::default(); 1],
        };

        let hdc = GetDC(None);
        let hbitmap = CreateDIBitmap(
            hdc,
            Some(&bmi.bmiHeader),
            CBM_INIT as u32,
            Some(img.as_raw().as_ptr() as *const _),
            Some(&mut bmi),
            DIB_RGB_COLORS,
        );

        ReleaseDC(None, hdc);
        hbitmap
    }
}
