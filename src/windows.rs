use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{AddClipboardFormatListener, RemoveClipboardFormatListener, GetMessageW, MSG, CreateWindowExW, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, WM_CLIPBOARDUPDATE};
use winapi::shared::minwindef::LPVOID;
use std::error::Error;
use crate::ClipboardCallback;

pub fn listen_clipboard(callback: ClipboardCallback) -> Result<(), Box<dyn Error>> {
    unsafe {
        let class_name: Vec<u16> = OsStr::new("STATIC").encode_wide().chain(Some(0)).collect();
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            ptr::null(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut() as LPVOID,
        );

        if hwnd.is_null() {
            return Err("Failed to create window.".into());
        }

        if AddClipboardFormatListener(hwnd) == 0 {
            return Err("Failed to add clipboard format listener.".into());
        }

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            if msg.message == WM_CLIPBOARDUPDATE {
                callback();
            }
            winapi::um::winuser::TranslateMessage(&msg);
            winapi::um::winuser::DispatchMessageW(&msg);
        }

        RemoveClipboardFormatListener(hwnd);
    }
    Ok(())
}