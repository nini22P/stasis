use fltk::prelude::*;
use fltk::{app, dialog, window::Window};
use fltk_webview::{FromFltkWindow, Webview};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::assets::{Assets, Locales};

#[derive(Serialize, Deserialize)]
struct Config {
    url: String,
}

fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("stasis");
        fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    })
}

pub fn save_url(url: &str) -> Result<(), std::io::Error> {
    if let Some(path) = get_config_path() {
        let config = Config {
            url: url.to_string(),
        };
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(path, json)?;
    }
    Ok(())
}

pub fn load_url() -> Option<String> {
    if let Some(path) = get_config_path() {
        if let Ok(data) = fs::read_to_string(path) {
            let config: Result<Config, _> = serde_json::from_str(&data);
            return config.ok().map(|c| c.url).filter(|url| !url.is_empty());
        }
    }
    None
}

pub fn run(is_modal: bool) {
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(420, 220)
        .center_screen()
        .with_label(&t!("config-title"));

    if is_modal {
        win.make_modal(true);
    }

    win.end();
    win.show();

    let wv = Webview::create(false, &mut win);

    wv.bind("ready", |_, _| {
        let locale = rust_i18n::locale();
        let file_name = format!("{}.json", &*locale);

        let content = Locales::get(&file_name).or_else(|| Locales::get("en.json"));

        if let Some(file_content) = content {
            if let Ok(json_string) = std::str::from_utf8(&file_content.data) {
                wv.eval(&format!("window.applyI18n({})", json_string));
            }
        }

        if let Some(url) = load_url() {
            let safe_url = url
                .replace('\\', "\\\\")
                .replace('\'', "\\'")
                .replace('"', "\\\"");
            wv.eval(&format!("window.loadConfig('{}')", safe_url));
        }
    });

    wv.bind("choose", |seq, _content| {
        let mut dialog = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
        dialog.set_filter("*.{htm,html}");
        dialog.show();
        let filename = dialog.filename().to_string_lossy().to_string();

        let result = format!("\"{}\"", filename.replace('\\', "\\\\"));
        wv.return_(seq, 0, &result);
    });

    wv.bind("save", |seq, content| {
        let mut status = "fail";
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(content) {
            if let Some(url_to_save) = parsed.get(0) {
                if let Err(e) = save_url(url_to_save) {
                    eprintln!("Error saving config: {}", e);
                } else {
                    status = "success";
                }
            }
        }

        wv.return_(seq, 0, &format!("\"{}\"", status));

        app.quit();
    });

    wv.bind("close", |_seq, _content| {
        app.quit();
    });

    if let Some(content) = Assets::get("config.html") {
        if let Ok(html_str) = std::str::from_utf8(&content.data) {
            let encoded_html = urlencoding::encode(html_str);
            let data_uri = format!("data:text/html;charset=utf-8,{}", encoded_html);
            wv.navigate(&data_uri);
        }
    } else {
        eprintln!("Fatal: Bundled 'config.html' not found!");
    }

    app.run().unwrap();
}
