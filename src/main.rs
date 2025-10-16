#![windows_subsystem = "windows"]

mod assets;
mod config;
mod screen_saver;

#[cfg(windows)]
mod winapi_utils;

rust_i18n::i18n!("locales");

enum Mode {
    Config(bool),
    Preview(isize),
    ScreenSaver,
}

fn main() {
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
    rust_i18n::set_locale(&locale);

    let args: Vec<String> = std::env::args().collect();
    let mut mode = Mode::Config(false);

    if let Some(arg1) = args.get(1) {
        let arg1_lower = arg1.to_lowercase();
        match arg1_lower.as_str() {
            "/c" => mode = Mode::Config(true),
            "/s" => mode = Mode::ScreenSaver,
            "/p" => {
                if let Some(hwnd_str) = args.get(2) {
                    if let Ok(hwnd) = hwnd_str.parse::<isize>() {
                        mode = Mode::Preview(hwnd);
                    }
                }
            }
            _ => {}
        }
    }

    match mode {
        Mode::Config(is_modal) => config::run(is_modal),
        Mode::Preview(hwnd) => screen_saver::run(Some(hwnd)),
        Mode::ScreenSaver => screen_saver::run(None),
    }
}
