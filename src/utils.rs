use windows_sys::Win32::{
    Foundation::{HWND, RECT},
    UI::{
        HiDpi::GetDpiForSystem,
        WindowsAndMessaging::{
            GWL_EXSTYLE, GWL_STYLE, GetClientRect, GetSystemMetrics, GetWindowLongPtrA,
            GetWindowLongPtrW, HWND_TOP, LWA_ALPHA, SM_CXSCREEN, SM_CYSCREEN, SW_MAXIMIZE,
            SWP_NOMOVE, SWP_NOZORDER, SWP_SHOWWINDOW, SetLayeredWindowAttributes, SetParent,
            SetWindowLongPtrA, SetWindowLongPtrW, SetWindowPos, ShowWindow, WS_CHILD,
            WS_EX_LAYERED, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_VISIBLE,
        },
    },
};

pub fn get_screen_size() -> (i32, i32) {
    unsafe {
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        (screen_width, screen_height)
    }
}

pub fn set_window_size(hwnd: HWND, width: i32, height: i32, center: bool, top: bool) {
    if hwnd.is_null() {
        return;
    }

    let mut x = 0;
    let mut y = 0;

    let mut flags = SWP_SHOWWINDOW;

    if center {
        unsafe {
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            let dpi = GetDpiForSystem() as i32;

            x = (screen_width - width) / 2 / dpi * 96;
            y = (screen_height - height) / 2 / dpi * 96;
        }
    } else {
        flags |= SWP_NOMOVE;
    }

    let hwnd_insert_after = if top {
        HWND_TOP
    } else {
        flags |= SWP_NOZORDER;
        0 as HWND
    };

    unsafe {
        SetWindowPos(hwnd, hwnd_insert_after, x, y, width, height, flags);
    }
}

pub fn set_fullscreen(hwnd: HWND) {
    if hwnd.is_null() {
        return;
    }

    unsafe {
        let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        SetWindowLongPtrW(
            hwnd,
            GWL_STYLE,
            (style & !WS_OVERLAPPEDWINDOW as isize) | WS_POPUP as isize,
        );
        ShowWindow(hwnd, SW_MAXIMIZE);
    }
}

pub fn set_as_child(hwnd: HWND, parent_hwnd: HWND) {
    if hwnd.is_null() || parent_hwnd.is_null() {
        return;
    }

    unsafe {
        SetParent(hwnd, parent_hwnd);

        let style = GetWindowLongPtrA(hwnd, GWL_STYLE) as u32;
        let new_style = (style & !WS_OVERLAPPEDWINDOW) | WS_CHILD | WS_VISIBLE;
        SetWindowLongPtrA(hwnd, GWL_STYLE, new_style as isize);

        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(parent_hwnd as HWND, &mut rect);
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        SetWindowPos(
            hwnd,
            0 as HWND,
            0,
            0,
            width,
            height,
            SWP_NOZORDER | SWP_SHOWWINDOW,
        );
    }
}

pub fn set_window_alpha(hwnd: HWND, alpha: u8) {
    if hwnd.is_null() {
        return;
    }

    unsafe {
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED as isize);
        SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA);
    }
}
