#![cfg(windows)]

use std::ffi::c_void;
use windows_sys::Win32::{
    Foundation::{HWND, RECT},
    UI::WindowsAndMessaging::{
        GWL_STYLE, GetClientRect, GetWindowLongPtrA, SWP_NOZORDER, SWP_SHOWWINDOW, SetParent,
        SetWindowLongPtrA, SetWindowPos, WS_CHILD, WS_VISIBLE,
    },
};

pub fn set_as_child(win_handle_ptr: *mut c_void, parent_hwnd: isize) {
    if win_handle_ptr.is_null() || parent_hwnd == 0 {
        return;
    }

    let win_handle = win_handle_ptr as HWND;

    unsafe {
        SetParent(win_handle, parent_hwnd as HWND);

        let style = GetWindowLongPtrA(win_handle, GWL_STYLE) as u32;
        let new_style =
            (style & !0x00C00000 & !0x00800000 & !0x00400000 & !0x00080000 & !0x00040000)
                | WS_CHILD
                | WS_VISIBLE;
        SetWindowLongPtrA(win_handle, GWL_STYLE, new_style as isize);

        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(parent_hwnd as HWND, &mut rect);
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        SetWindowPos(
            win_handle,
            0 as HWND,
            0,
            0,
            width,
            height,
            SWP_NOZORDER | SWP_SHOWWINDOW,
        );
    }
}
