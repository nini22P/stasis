use fltk::{app::App, enums::Event, prelude::*, window::Window};
use fltk_webview::{FromFltkWindow, Webview};

use crate::{assets::Assets, config};

#[cfg(windows)]
use crate::winapi_utils;

pub fn run(parent_hwnd: Option<isize>) {
    let app = App::default();
    let mut win = Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("Stasis");

    win.set_color(fltk::enums::Color::Black);
    win.make_resizable(true);

    if parent_hwnd.is_none() {
        win.fullscreen(true);
    }

    win.end();
    win.show();

    #[cfg(windows)]
    if let Some(hwnd) = parent_hwnd {
        let win_handle_ptr = win.raw_handle();
        winapi_utils::set_as_child(win_handle_ptr, hwnd);
    }

    win.set_opacity(0.0);

    let wv = Webview::create(false, &mut win);

    win.handle(move |_, ev| match ev {
        Event::KeyDown => {
            if parent_hwnd.is_none() {
                app.quit();
            }
            true
        }
        _ => false,
    });

    wv.bind("ready", |_, _| {
        win.set_opacity(1.0);
    });

    wv.bind("close", |_, _| {
        if parent_hwnd.is_none() {
            app.quit();
        }
    });

    if let Some(content) = Assets::get("init.js") {
        wv.init(&String::from_utf8_lossy(&content.data));
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
                        wv.navigate(&data_uri);
                    } else {
                        eprintln!("Fatal: Bundled screensaver not valid UTF-8!");
                    }
                }
            } else {
                wv.navigate(&uri);
            }
        }
        None => {
            if let Some(content) = Assets::get("screensavers/default/index.html") {
                if let Ok(html_str) = std::str::from_utf8(&content.data) {
                    let encoded_html = urlencoding::encode(html_str);
                    let data_uri = format!("data:text/html;charset=utf-8,{}", encoded_html);
                    wv.navigate(&data_uri);
                }
            } else {
                eprintln!("Fatal: Bundled 'default.html' not found!");
            }
        }
    }

    app.run().unwrap();
}
