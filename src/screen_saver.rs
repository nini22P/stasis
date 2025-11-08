use windows_sys::Win32::Foundation::HWND;
use wv::{SizeHint, Webview};

use crate::{
    assets::Assets,
    config,
    utils::{get_screen_size, set_as_child, set_fullscreen, set_window_alpha, set_window_size},
};

pub fn run(parent_hwnd: Option<isize>) {
    unsafe extern "C" {
        pub fn move_focus(wv: *mut wv::webview_t);
    }

    let mut wv = Webview::create_no_win(true);

    let hwnd = wv.get_window();

    set_window_alpha(hwnd, 0);

    let (screen_width, screen_height) = get_screen_size();

    set_window_size(hwnd, screen_width, screen_height, true, true);

    wv.set_size(screen_width, screen_height, SizeHint::None)
        .unwrap();
    wv.set_title("Stasis").unwrap();

    if parent_hwnd.is_none() {
        set_fullscreen(hwnd);
    }

    if let Some(parent_hwnd) = parent_hwnd {
        set_as_child(hwnd, parent_hwnd as HWND);
    }

    wv.bind("ready", |_, _| {
        set_window_alpha(hwnd, 255);
        unsafe { move_focus(*wv.inner as *mut _) };
    })
    .unwrap();

    wv.bind("quit", |_, _| {
        if parent_hwnd.is_none() {
            std::process::exit(0);
        }
    })
    .unwrap();

    if let Some(content) = Assets::get("init.js") {
        wv.init(&String::from_utf8_lossy(&content.data)).unwrap();
    }

    let stored_config = config::load_config();
    let uri_to_load = if stored_config.selected_uri.is_empty() {
        None
    } else {
        Some(stored_config.selected_uri)
    };

    match uri_to_load {
        Some(uri) => {
            if uri.starts_with("screensavers/") {
                if let Some(content) = Assets::get(&uri) {
                    if let Ok(html_str) = std::str::from_utf8(&content.data) {
                        let encoded_html = urlencoding::encode(html_str);
                        let data_uri = format!("data:text/html;charset=utf-8,{}", encoded_html);
                        wv.navigate(&data_uri).unwrap();
                    } else {
                        eprintln!("Fatal: Bundled screensaver not valid UTF-8!");
                    }
                }
            } else {
                wv.navigate(&uri).unwrap();
            }
        }
        None => {
            if let Some(content) = Assets::get("screensavers/default/index.html") {
                if let Ok(html_str) = std::str::from_utf8(&content.data) {
                    let encoded_html = urlencoding::encode(html_str);
                    let data_uri = format!("data:text/html;charset=utf-8,{}", encoded_html);
                    wv.navigate(&data_uri).unwrap();
                }
            } else {
                eprintln!("Fatal: Bundled 'default.html' not found!");
            }
        }
    }

    wv.run().unwrap();
}
